use serde_json::Value;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct BookDepth {
    pub best_bid: f64,
    pub best_ask: f64,
    pub spread: f64,
    pub bid_depth: f64, // total volume on bid side
    pub ask_depth: f64, // total volume on ask side
    pub imbalance: f64, // (bid_depth - ask_depth) / (bid_depth + ask_depth), range -1 to 1
}

/// Fetch order book depth from Kraken CLI.
pub fn get_book_depth(pair: &str, depth: usize) -> anyhow::Result<BookDepth> {
    let output = Command::new("kraken")
        .args(["book", pair, "--depth", &depth.to_string(), "-o", "json"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "kraken book failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    parse_book_depth(&output.stdout)
}

fn parse_book_depth(stdout: &[u8]) -> anyhow::Result<BookDepth> {
    let root: Value = serde_json::from_slice(stdout)?;

    // Find the pair object containing bids/asks
    let obj = root
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("book: expected object"))?;

    let pair_data = obj
        .values()
        .find(|v| v.get("bids").is_some() || v.get("asks").is_some())
        .ok_or_else(|| anyhow::anyhow!("book: no bid/ask data"))?;

    let bids = pair_data
        .get("bids")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("book: no bids array"))?;
    let asks = pair_data
        .get("asks")
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("book: no asks array"))?;

    let best_bid = parse_level_price(bids.first())?;
    let best_ask = parse_level_price(asks.first())?;

    let bid_depth: f64 = bids.iter().filter_map(parse_level_volume).sum();
    let ask_depth: f64 = asks.iter().filter_map(parse_level_volume).sum();

    let total = bid_depth + ask_depth;
    let imbalance = if total > 0.0 {
        (bid_depth - ask_depth) / total
    } else {
        0.0
    };

    Ok(BookDepth {
        best_bid,
        best_ask,
        spread: best_ask - best_bid,
        bid_depth,
        ask_depth,
        imbalance,
    })
}

fn parse_level_price(level: Option<&Value>) -> anyhow::Result<f64> {
    let arr = level
        .and_then(|v| v.as_array())
        .ok_or_else(|| anyhow::anyhow!("book: empty level"))?;
    let price_str = arr
        .first()
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("book: no price"))?;
    Ok(price_str.parse()?)
}

fn parse_level_volume(level: &Value) -> Option<f64> {
    let arr = level.as_array()?;
    let vol_str = arr.get(1)?.as_str()?;
    vol_str.parse().ok()
}
