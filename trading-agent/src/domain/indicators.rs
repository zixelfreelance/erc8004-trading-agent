use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdResult {
    pub macd_line: f64,
    pub signal_line: f64,
    pub histogram: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBands {
    pub upper: f64,
    pub middle: f64,
    pub lower: f64,
    pub bandwidth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdxResult {
    pub adx: f64,
    pub plus_di: f64,
    pub minus_di: f64,
}

/// Simple moving average of the last `period` values.
pub fn sma(data: &[f64], period: usize) -> Option<f64> {
    if period == 0 || data.len() < period {
        return None;
    }
    let sum: f64 = data[data.len() - period..].iter().sum();
    Some(sum / period as f64)
}

/// Exponential moving average. Returns a Vec starting from index `period - 1`.
pub fn ema(data: &[f64], period: usize) -> Option<Vec<f64>> {
    if period == 0 || data.len() < period {
        return None;
    }
    let multiplier = 2.0 / (period as f64 + 1.0);
    let first_sma: f64 = data[..period].iter().sum::<f64>() / period as f64;

    let mut result = Vec::with_capacity(data.len() - period + 1);
    result.push(first_sma);

    for &val in &data[period..] {
        let prev = *result.last().unwrap();
        result.push((val - prev) * multiplier + prev);
    }
    Some(result)
}

/// Relative Strength Index (0–100).
pub fn rsi(closes: &[f64], period: usize) -> Option<f64> {
    if period == 0 || closes.len() < period + 1 {
        return None;
    }

    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;

    // Initial averages over first `period` changes
    for i in 1..=period {
        let change = closes[i] - closes[i - 1];
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss += change.abs();
        }
    }
    avg_gain /= period as f64;
    avg_loss /= period as f64;

    // Smooth over remaining data
    for i in (period + 1)..closes.len() {
        let change = closes[i] - closes[i - 1];
        let (gain, loss) = if change > 0.0 {
            (change, 0.0)
        } else {
            (0.0, change.abs())
        };
        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;
    }

    if avg_loss == 0.0 {
        return Some(100.0);
    }
    let rs = avg_gain / avg_loss;
    Some(100.0 - (100.0 / (1.0 + rs)))
}

/// MACD with configurable fast/slow/signal periods.
pub fn macd(closes: &[f64], fast: usize, slow: usize, signal: usize) -> Option<MacdResult> {
    if closes.len() < slow + signal {
        return None;
    }

    let fast_ema = ema(closes, fast)?;
    let slow_ema = ema(closes, slow)?;

    // Align: fast_ema starts at index (fast-1), slow_ema at (slow-1).
    // We need the overlapping region.
    let offset = slow - fast; // how many more elements fast_ema has
    let macd_values: Vec<f64> = fast_ema[offset..]
        .iter()
        .zip(slow_ema.iter())
        .map(|(f, s)| f - s)
        .collect();

    if macd_values.len() < signal {
        return None;
    }

    let signal_ema = ema(&macd_values, signal)?;
    let macd_line = *macd_values.last().unwrap();
    let signal_line = *signal_ema.last().unwrap();

    Some(MacdResult {
        macd_line,
        signal_line,
        histogram: macd_line - signal_line,
    })
}

/// Bollinger Bands with configurable period and standard deviation multiplier.
pub fn bollinger(closes: &[f64], period: usize, num_std: f64) -> Option<BollingerBands> {
    if period == 0 || closes.len() < period {
        return None;
    }

    let window = &closes[closes.len() - period..];
    let middle = window.iter().sum::<f64>() / period as f64;
    let variance = window.iter().map(|x| (x - middle).powi(2)).sum::<f64>() / period as f64;
    let stddev = variance.sqrt();

    let upper = middle + num_std * stddev;
    let lower = middle - num_std * stddev;
    let bandwidth = if middle != 0.0 {
        (upper - lower) / middle
    } else {
        0.0
    };

    Some(BollingerBands {
        upper,
        middle,
        lower,
        bandwidth,
    })
}

/// Average True Range over `period`.
pub fn atr(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<f64> {
    let len = highs.len();
    if period == 0 || len < period + 1 || lows.len() != len || closes.len() != len {
        return None;
    }

    let mut tr_values = Vec::with_capacity(len - 1);
    for i in 1..len {
        let hl = highs[i] - lows[i];
        let hc = (highs[i] - closes[i - 1]).abs();
        let lc = (lows[i] - closes[i - 1]).abs();
        tr_values.push(hl.max(hc).max(lc));
    }

    // SMA of the last `period` true range values
    if tr_values.len() < period {
        return None;
    }
    let sum: f64 = tr_values[tr_values.len() - period..].iter().sum();
    Some(sum / period as f64)
}

/// Average Directional Index.
pub fn adx(highs: &[f64], lows: &[f64], closes: &[f64], period: usize) -> Option<AdxResult> {
    let len = highs.len();
    if period == 0 || len < 2 * period + 1 || lows.len() != len || closes.len() != len {
        return None;
    }

    // Compute True Range, +DM, -DM
    let n = len - 1; // number of bars with movement data
    let mut tr = Vec::with_capacity(n);
    let mut plus_dm = Vec::with_capacity(n);
    let mut minus_dm = Vec::with_capacity(n);

    for i in 1..len {
        let hl = highs[i] - lows[i];
        let hc = (highs[i] - closes[i - 1]).abs();
        let lc = (lows[i] - closes[i - 1]).abs();
        tr.push(hl.max(hc).max(lc));

        let up = highs[i] - highs[i - 1];
        let down = lows[i - 1] - lows[i];

        if up > down && up > 0.0 {
            plus_dm.push(up);
        } else {
            plus_dm.push(0.0);
        }
        if down > up && down > 0.0 {
            minus_dm.push(down);
        } else {
            minus_dm.push(0.0);
        }
    }

    // Wilder smoothing: first value = sum of first `period`, then smooth
    let smooth = |data: &[f64]| -> Vec<f64> {
        let mut result = Vec::with_capacity(data.len() - period + 1);
        let first: f64 = data[..period].iter().sum();
        result.push(first);
        for &val in &data[period..] {
            let prev = *result.last().unwrap();
            result.push(prev - prev / period as f64 + val);
        }
        result
    };

    let smoothed_tr = smooth(&tr);
    let smoothed_plus_dm = smooth(&plus_dm);
    let smoothed_minus_dm = smooth(&minus_dm);

    // +DI and -DI series
    let di_len = smoothed_tr.len();
    let mut dx_values = Vec::with_capacity(di_len);
    let mut last_plus_di = 0.0;
    let mut last_minus_di = 0.0;

    for i in 0..di_len {
        let str_val = smoothed_tr[i];
        if str_val == 0.0 {
            dx_values.push(0.0);
            continue;
        }
        let pdi = 100.0 * smoothed_plus_dm[i] / str_val;
        let mdi = 100.0 * smoothed_minus_dm[i] / str_val;
        last_plus_di = pdi;
        last_minus_di = mdi;

        let di_sum = pdi + mdi;
        if di_sum == 0.0 {
            dx_values.push(0.0);
        } else {
            dx_values.push(100.0 * (pdi - mdi).abs() / di_sum);
        }
    }

    if dx_values.len() < period {
        return None;
    }

    // ADX = Wilder-smoothed DX
    let mut adx_val: f64 = dx_values[..period].iter().sum::<f64>() / period as f64;
    for &dx in &dx_values[period..] {
        adx_val = (adx_val * (period as f64 - 1.0) + dx) / period as f64;
    }

    Some(AdxResult {
        adx: adx_val,
        plus_di: last_plus_di,
        minus_di: last_minus_di,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma_basic() {
        let data = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(sma(&data, 3), Some(4.0));
    }

    #[test]
    fn test_ema_basic() {
        let data: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = ema(&data, 5).unwrap();
        // EMA of 20 values with period 5 → 20 - 5 + 1 = 16 values
        assert_eq!(result.len(), 16);
    }

    #[test]
    fn test_rsi_overbought() {
        // Strongly rising prices
        let closes: Vec<f64> = (0..30).map(|i| 100.0 + i as f64 * 5.0).collect();
        let r = rsi(&closes, 14).unwrap();
        assert!(r > 70.0, "RSI should be > 70 for rising prices, got {r}");
    }

    #[test]
    fn test_rsi_oversold() {
        // Strongly falling prices
        let closes: Vec<f64> = (0..30).map(|i| 200.0 - i as f64 * 5.0).collect();
        let r = rsi(&closes, 14).unwrap();
        assert!(r < 30.0, "RSI should be < 30 for falling prices, got {r}");
    }

    #[test]
    fn test_rsi_neutral() {
        // Alternating up/down around a center → near 50
        let mut closes = Vec::new();
        for i in 0..30 {
            closes.push(100.0 + if i % 2 == 0 { 1.0 } else { -1.0 });
        }
        let r = rsi(&closes, 14).unwrap();
        assert!(
            (40.0..=60.0).contains(&r),
            "RSI should be near 50 for flat prices, got {r}"
        );
    }

    #[test]
    fn test_macd_uptrend() {
        // Strong uptrend needs enough data for slow EMA to converge
        let closes: Vec<f64> = (0..80).map(|i| 100.0 + i as f64 * 2.0).collect();
        let result = macd(&closes, 12, 26, 9).unwrap();
        assert!(
            result.macd_line > 0.0,
            "MACD line should be positive in uptrend, got {}",
            result.macd_line
        );
    }

    #[test]
    fn test_bollinger_bands_valid() {
        let closes: Vec<f64> = (0..30).map(|i| 100.0 + (i as f64 * 0.1).sin() * 5.0).collect();
        let bb = bollinger(&closes, 20, 2.0).unwrap();
        assert!(bb.upper > bb.middle, "Upper band should be above middle");
        assert!(bb.middle > bb.lower, "Middle should be above lower band");
        assert!(bb.bandwidth > 0.0, "Bandwidth should be positive");
    }

    #[test]
    fn test_atr_positive() {
        let n = 20;
        let highs: Vec<f64> = (0..n).map(|i| 105.0 + i as f64).collect();
        let lows: Vec<f64> = (0..n).map(|i| 95.0 + i as f64).collect();
        let closes: Vec<f64> = (0..n).map(|i| 100.0 + i as f64).collect();
        let result = atr(&highs, &lows, &closes, 14).unwrap();
        assert!(result > 0.0, "ATR should be positive, got {result}");
    }

    #[test]
    fn test_adx_trending() {
        // Strong uptrend: consistently higher highs and higher lows
        let n = 50;
        let highs: Vec<f64> = (0..n).map(|i| 110.0 + i as f64 * 3.0).collect();
        let lows: Vec<f64> = (0..n).map(|i| 100.0 + i as f64 * 3.0).collect();
        let closes: Vec<f64> = (0..n).map(|i| 105.0 + i as f64 * 3.0).collect();
        let result = adx(&highs, &lows, &closes, 14).unwrap();
        assert!(
            result.adx > 25.0,
            "ADX should be > 25 for trending data, got {}",
            result.adx
        );
    }

    #[test]
    fn test_insufficient_data_returns_none() {
        let short = [1.0, 2.0];
        assert!(sma(&short, 5).is_none());
        assert!(ema(&short, 5).is_none());
        assert!(rsi(&short, 14).is_none());
        assert!(macd(&short, 12, 26, 9).is_none());
        assert!(bollinger(&short, 20, 2.0).is_none());
        assert!(atr(&short, &short, &short, 14).is_none());
        assert!(adx(&short, &short, &short, 14).is_none());

        // Empty slice
        let empty: &[f64] = &[];
        assert!(sma(empty, 1).is_none());
        assert!(ema(empty, 1).is_none());
        assert!(rsi(empty, 1).is_none());
    }
}
