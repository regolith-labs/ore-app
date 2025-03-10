use dioxus::document::eval;
use dioxus::prelude::*;

const BASE_URL: &str = "https://ore-app-xyz.s3.amazonaws.com/";
const DEFAULT_OS: &str = "macos";

pub fn use_download_url() -> Signal<DownloadUrl> {
    use_context()
}

pub type DownloadUrl = String;
pub fn use_download_url_provider() {
    let default_url = artifact(DEFAULT_OS, "arm64");
    let mut signal = use_context_provider::<Signal<DownloadUrl>>(|| Signal::new(default_url));
    let mut eval = eval(
        r#"
        function guessOsAndArch() {
            const ua = navigator.userAgent || "";
            const platform = navigator.platform || "";

            console.log("ua: ", ua);
            console.log("platform: ", platform);
            
            // Detect OS
            let os = "unknown";
            if (/Mac|iPhone|iPad|iPod/.test(platform)) {
                os = "macos";
            } else if (/Win/.test(platform)) {
                os = "windows"; 
            } else if (/Linux/.test(platform)) {
                os = "linux";
            }

            // Detect architecture
            let arch = "unknown";
            
            // Chrome on M1/M2 Macs runs under Rosetta 2 by default, so we need additional checks
            if (/Mac/.test(platform)) {
                // Check if running on Apple Silicon via canvas
                const canvas = document.createElement('canvas');
                const gl = canvas.getContext('webgl');
                const debugInfo = gl.getExtension('WEBGL_debug_renderer_info');
                const renderer = gl.getParameter(debugInfo.UNMASKED_RENDERER_WEBGL);
                
                if (renderer.includes('Apple')) {
                    // Apple GPU indicates Apple Silicon
                    arch = "arm64";
                } else {
                    arch = "x86_64";
                }
            } else {
                // For non-Macs, use standard detection
                if (/\b(arm64|aarch64)\b/i.test(ua)) {
                    arch = "arm64";
                } else if (/\b(x86_64|amd64|x86-64|x64)\b/i.test(ua) || 
                        /\b(Win64|WOW64)\b/i.test(ua)) {
                    arch = "x86_64";
                }
            }

            return {os, arch};
        }
        const result = guessOsAndArch();
        dioxus.send(JSON.stringify(result));
        "#,
    );
    spawn(async move {
        match eval.recv::<String>().await {
            Ok(arch) => {
                // log::info!("arch: {}", arch);
                // let arch = arch_from_string(arch.as_str());
                let result: serde_json::Value = serde_json::from_str(&arch).unwrap();
                let os = result["os"].as_str().unwrap_or("unknown");
                let arch = result["arch"].as_str().unwrap_or("unknown");

                log::info!("os: {}", os);
                log::info!("arch: {}", arch);

                let artifact = artifact(os, arch);
                signal.set(artifact);
            }
            Err(err) => {
                log::error!("{:?}", err);
            }
        };
    });
}

fn artifact(os: &str, arch: &str) -> String {
    // Map architecture names
    let name = match arch {
        "x86_64" => "x86_64",
        "arm64" => "aarch64",
        _ => "unknown",
    };

    // Normalize architecture for filename
    let normalized = match name {
        "x86_64" => "x64",
        _ => name,
    };

    // Generate platform-specific extension
    let ext = match os {
        "windows" => "exe",
        "macos" => "dmg",
        "linux" => "AppImage",
        _ => "unknown",
    };

    format!(
        "{}{}/{}/latest/ore_latest_{}.{}",
        BASE_URL, os, name, normalized, ext
    )
}

pub fn parse_download_url(url: &String) -> (String, String) {
    // Split URL into parts and extract OS and architecture
    let parts: Vec<&str> = url.split('/').collect();
    if parts.len() < 4 {
        return ("unknown".to_string(), "unknown".to_string());
    }

    // Get OS from URL path
    let os = match parts[3] {
        "windows" => "Windows",
        "macos" => "macOS",
        "linux" => "Linux",
        os => os,
    }
    .to_string();

    // Get architecture - parts[4] contains the raw architecture
    let arch = match parts[4] {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        arch => arch,
    }
    .to_string();

    (os, arch)
}
