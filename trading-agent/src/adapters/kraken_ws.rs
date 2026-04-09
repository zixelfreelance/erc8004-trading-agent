use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde_json::Value;

/// Real-time price from Kraken WebSocket.
#[derive(Debug, Clone)]
pub struct WsTick {
    pub price: f64,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub volume: Option<f64>,
}

/// Manages a background `kraken ws` subprocess and provides the latest tick.
/// Automatically reconnects with exponential backoff on disconnect.
pub struct KrakenWsStream {
    latest: Arc<Mutex<Option<WsTick>>>,
    _child: Option<Child>,
}

impl KrakenWsStream {
    /// Start streaming ticker data for a pair. Spawns a background thread
    /// that reconnects automatically on disconnect.
    pub fn start(pair: &str) -> Self {
        let latest: Arc<Mutex<Option<WsTick>>> = Arc::new(Mutex::new(None));
        let latest_clone = Arc::clone(&latest);

        let ws_pair = Self::format_ws_pair(pair);

        // Spawn a reconnecting reader thread
        thread::spawn(move || {
            Self::reconnect_loop(&ws_pair, &latest_clone);
        });

        Self {
            latest,
            _child: None,
        }
    }

    fn reconnect_loop(ws_pair: &str, latest: &Arc<Mutex<Option<WsTick>>>) {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(60);

        loop {
            match Command::new("kraken")
                .args(["ws", "ticker", ws_pair, "-o", "json"])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(mut child) => {
                    let stdout = match child.stdout.take() {
                        Some(s) => s,
                        None => {
                            eprintln!("kraken-ws: no stdout pipe");
                            thread::sleep(backoff);
                            continue;
                        }
                    };

                    // Reset backoff on successful connection
                    backoff = Duration::from_secs(1);
                    eprintln!("kraken-ws: connected to {ws_pair}");

                    let reader = BufReader::new(stdout);
                    for line in reader.lines() {
                        let Ok(line) = line else { break };
                        if let Ok(tick) = Self::parse_ws_tick(&line) {
                            if let Ok(mut guard) = latest.lock() {
                                *guard = Some(tick);
                            }
                        }
                    }

                    // Stream ended — kill child and reconnect
                    let _ = child.kill();
                    let _ = child.wait();
                    eprintln!(
                        "kraken-ws: disconnected, reconnecting in {}s",
                        backoff.as_secs()
                    );
                }
                Err(e) => {
                    eprintln!(
                        "kraken-ws: spawn failed: {e}, retrying in {}s",
                        backoff.as_secs()
                    );
                }
            }

            thread::sleep(backoff);
            backoff = (backoff * 2).min(max_backoff);
        }
    }

    /// Create a no-op instance.
    pub fn noop() -> Self {
        Self {
            latest: Arc::new(Mutex::new(None)),
            _child: None,
        }
    }

    pub fn is_running(&self) -> bool {
        // Check if latest tick has been received (stream is active)
        self.latest
            .lock()
            .map(|g| g.is_some())
            .unwrap_or(false)
    }

    /// Get the latest tick (if any). Returns None if no data received yet.
    pub fn latest_tick(&self) -> Option<WsTick> {
        self.latest.lock().ok()?.clone()
    }

    fn format_ws_pair(pair: &str) -> String {
        // Convert "BTCUSD" to "BTC/USD"
        if pair.len() == 6 && !pair.contains('/') {
            format!("{}/{}", &pair[..3], &pair[3..])
        } else {
            pair.to_string()
        }
    }

    fn parse_ws_tick(line: &str) -> anyhow::Result<WsTick> {
        let v: Value = serde_json::from_str(line)?;

        // Kraken WS ticker format varies, try common fields
        let price = Self::extract_f64(&v, &["c", "last", "price"])
            .ok_or_else(|| anyhow::anyhow!("no price in ws tick"))?;
        let bid = Self::extract_f64(&v, &["b", "bid"]);
        let ask = Self::extract_f64(&v, &["a", "ask"]);
        let volume = Self::extract_f64(&v, &["v", "volume"]);

        Ok(WsTick {
            price,
            bid,
            ask,
            volume,
        })
    }

    fn extract_f64(v: &Value, keys: &[&str]) -> Option<f64> {
        for key in keys {
            if let Some(val) = v.get(key) {
                // Could be string or number, could be array where first element is the value
                if let Some(n) = val.as_f64() {
                    return Some(n);
                }
                if let Some(s) = val.as_str() {
                    if let Ok(n) = s.parse::<f64>() {
                        return Some(n);
                    }
                }
                if let Some(arr) = val.as_array() {
                    if let Some(first) = arr.first() {
                        if let Some(s) = first.as_str() {
                            if let Ok(n) = s.parse::<f64>() {
                                return Some(n);
                            }
                        }
                        if let Some(n) = first.as_f64() {
                            return Some(n);
                        }
                    }
                }
            }
        }
        None
    }
}
