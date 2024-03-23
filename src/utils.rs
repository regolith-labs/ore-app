#[cfg(feature = "desktop")]
pub fn asset_path(relative_path: &str) -> String {
    if cfg!(feature = "bundle") {
        format!("public/{}", relative_path)
    } else {
        relative_path.to_string()
    }
}

#[cfg(feature = "web")]
pub fn asset_path(relative_path: &str) -> String {
    relative_path.to_string()
}
