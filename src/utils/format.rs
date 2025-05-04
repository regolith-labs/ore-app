use steel::Pubkey;

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

pub fn format_whole_number(amount_string: String) -> String {
    // Remove any decimal portion
    let whole_part = amount_string.split('.').next().unwrap_or(&amount_string);

    // Convert to numeric to remove any existing commas
    if let Ok(num) = whole_part.replace(',', "").parse::<u64>() {
        if num >= 1_000_000 {
            format!(
                "{},{:03},{:03}",
                num / 1_000_000,
                (num % 1_000_000) / 1000,
                num % 1000
            )
        } else if num >= 1_000 {
            format!("{},{:03}", num / 1000, num % 1000)
        } else {
            num.to_string()
        }
    } else {
        amount_string
    }
}

pub fn format_percentage(pct: f64) -> String {
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
    } else if diff < 86400 * 7 {
        let hours = diff / 3600;
        if hours == 1 {
            format!("1 hour")
        } else {
            format!("{} hours", hours)
        }
    } else {
        let days = diff / 86400;
        if days == 1 {
            format!("1 day")
        } else {
            format!("{} days", days)
        }
    }
}

pub fn _format_bps_as_percent(bps: f64) -> String {
    let percent = bps / ore_boost_api::consts::DENOMINATOR_BPS as f64 * 100.0;
    if percent < 0.1 {
        format!("{:.2}%", percent)
    } else {
        format!("{:.1}%", percent)
    }
}

pub fn format_abbreviated_pubkey(pubkey: Pubkey) -> String {
    let pubkey_str = pubkey.to_string();
    format!(
        "{}...{}",
        &pubkey_str[..4],
        &pubkey_str[pubkey_str.len() - 4..]
    )
}
