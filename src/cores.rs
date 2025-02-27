#[cfg(feature = "web")]
pub fn get() -> usize {
    use web_sys::window;
    // Get the Window
    let win = match window() {
        Some(win) => win,
        None => {
            return 1;
        }
    };
    // Get the Navigator
    let nav = win.navigator();
    // Get the hardware concurrency
    nav.hardware_concurrency() as usize
}

#[cfg(not(feature = "web"))]
pub fn get() -> usize {
    num_cpus::get()
}
