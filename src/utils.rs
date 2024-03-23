#[cfg(feature = "desktop")]
pub fn asset_path(relative_path: &str) -> String {
    format!("public/{}", relative_path)
}

#[cfg(not(feature = "desktop"))]
pub fn asset_path(relative_path: &str) -> String {
    relative_path.to_string()
}
