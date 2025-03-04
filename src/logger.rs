#[cfg(feature = "web")]
pub fn init() {
    wasm_logger::init(wasm_logger::Config::default());
}

#[cfg(not(feature = "web"))]
pub fn init() {
    env_logger::init();
}

// macos logger,
// use for debugging production apps with no dev server.
//
// cat ~/Library/Caches/ore.supply.ore-app/logs/ore.log
#[cfg(not(feature = "web"))]
pub fn _init_macos() {
    use directories::ProjectDirs;
    use fern::Dispatch;
    use log::LevelFilter;
    use std::fs;
    use std::path::PathBuf;
    let proj_dirs =
        ProjectDirs::from("ore", "supply", "ore-app").expect("Could not determine home directory");
    let log_dir: PathBuf = proj_dirs.cache_dir().join("logs");
    fs::create_dir_all(&log_dir).expect("Could not create log directory");
    let log_file_path = log_dir.join("ore.log");
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LevelFilter::Info)
        .chain(fern::log_file(&log_file_path).expect("Failed to open log file"))
        .apply()
        .expect("Failed to set up logging");
}
