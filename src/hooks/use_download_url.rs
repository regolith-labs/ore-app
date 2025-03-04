use anyhow::Result;
use dioxus::document::eval;
use dioxus::prelude::*;

const BASE_URL: &str = "https://ore-app-xyz.s3.amazonaws.com/";
const DEFAULT_OS: &str = "macos";

pub fn use_download_url() -> Signal<DownloadUrl> {
    use_context()
}

pub type DownloadUrl = String;
pub fn use_download_url_provider() {
    let default_url = artifact(&Arch::Arm64);
    let mut signal = use_context_provider::<Signal<DownloadUrl>>(|| Signal::new(default_url));
    let mut eval = eval(
        r#"
        function guessArchitecture() {
           const ua = navigator.userAgent || "";
           // Typical patterns for x86_64
           if (
             /\b(x86_64|amd64|x86-64)\b/i.test(ua) ||
             /\b(Win64|WOW64)\b/i.test(ua) ||
             /\bIntel Mac OS X\b/i.test(ua) // older Mac Intel machines
           ) {
             return "x86_64";
           // Common patterns for ARM64
           } else if (
             /\b(arm64|aarch64)\b/i.test(ua) ||
             /\bAndroid\b/i.test(ua) // many Android devices are ARM64
           ) {
             return "arm64";
           // Some older 32-bit ARM patterns
           } else if (
             /\barmv?7l?\b/i.test(ua) ||
             /\barmv?8l?\b/i.test(ua)
           ) {
             return "arm";
           } else {
             return "unknown";
           }
        }
        let arch = guessArchitecture();
        dioxus.send(arch);
        "#,
    );
    spawn(async move {
        match eval.recv::<String>().await {
            Ok(arch) => {
                log::info!("arch: {:?}", arch);
                let arch = arch_from_string(arch.as_str());
                let artifact = artifact(&arch);
                signal.set(artifact);
            }
            Err(err) => {
                log::error!("{:?}", err);
            }
        };
    });
}

enum Arch {
    X86_64,
    Arm64,
}

fn artifact(arch: &Arch) -> String {
    let name = match arch {
        Arch::X86_64 => "x86_64",
        Arch::Arm64 => "aarch64",
    };
    let normalized = match name {
        "x86_64" => "x64",
        _ => name,
    };
    format!(
        "{}{}/{}/latest/ore_latest_{}.dmg",
        BASE_URL, DEFAULT_OS, name, normalized
    )
}

fn arch_from_string(str: &str) -> Arch {
    match str {
        "x86_64" => Arch::X86_64,
        _ => Arch::Arm64,
    }
}
