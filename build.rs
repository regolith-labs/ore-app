fn main() {
    // Automatically enable macos-desktop feature when building on macOS with desktop feature
    #[cfg(all(target_os = "macos", feature = "desktop"))]
    println!("cargo:rustc-cfg=feature=\"macos-desktop\"");
}
