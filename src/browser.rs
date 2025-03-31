use std::path::Path;
use std::process::Command;

#[tauri::command]
fn open_in_chromium_or_fallback(url: &str) -> Result<(), String> {
    // Define the common install locations for Chromium-based browsers on macOS.
    let chrome_path = "/Applications/Google Chrome.app";
    let chromium_path = "/Applications/Chromium.app";

    // Determine which browser to use if installed.
    let browser = if Path::new(chrome_path).exists() {
        "Google Chrome"
    } else if Path::new(chromium_path).exists() {
        "Chromium"
    } else {
        // Fallback: open with the OS default browser.
        return open_in_default_browser(url);
    };

    // Attempt to open the URL using the detected Chromium-based browser.
    let status = Command::new("open")
        .args(&["-a", browser, url])
        .status()
        .map_err(|e| format!("Failed to execute open command: {}", e))?;

    if !status.success() {
        return Err(format!("Failed to open URL in {}", browser));
    }

    Ok(())
}

/// Opens the given URL in the default browser.
fn open_in_default_browser(url: &str) -> Result<(), String> {
    let status = Command::new("open")
        .arg(url)
        .status()
        .map_err(|e| format!("Failed to execute open command: {}", e))?;
    if !status.success() {
        return Err("Failed to open URL in the default browser".into());
    }
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_in_chromium_or_fallback])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
