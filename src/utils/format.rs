pub fn format_abbreviated_number(amount: f64) -> String {
    if amount >= 1_000_000_000.0 {
        format!("{:.1}B", amount / 1_000_000_000.0)
    } else if amount >= 1_000_000.0 {
        format!("{:.1}M", amount / 1_000_000.0)
    } else if amount >= 1_000.0 {
        format!("{:.1}k", amount / 1_000.0)
    } else {
        format!("{:.1}", amount)
    }
}

pub fn _format_percentage(pct: f64) -> String {
    let pct = if pct < 1.0 {
        // Find first non-zero decimal place
        let mut decimals = 0;
        let mut val = pct;
        while val < 1.0 {
            val *= 10.0;
            decimals += 1;
        }
        (pct * 10f64.powi(decimals)).floor() / 10f64.powi(decimals)
    } else {
        (pct * 10.0).floor() / 10.0 // One decimal place
    };
    format!("{:.1}%", pct)
}

pub fn format_time_since(timestamp: u64) -> String {
    let now = crate::time::SystemTime::now()
        .duration_since(crate::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let diff = now.saturating_sub(timestamp);

    if diff < 60 {
        format!("{} sec", diff)
    } else if diff < 3600 {
        format!("{} min", diff / 60)
    } else if diff < 86400 {
        format!("{} hours", diff / 3600)
    } else {
        format!("{} days", diff / 86400)
    }
}
