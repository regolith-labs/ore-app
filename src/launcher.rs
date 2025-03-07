use dioxus::prelude::*;

#[cfg(feature = "web")]
pub fn launch(app: fn() -> Element) {
    dioxus::launch(app)
}

#[cfg(not(feature = "web"))]
pub fn launch(app: fn() -> Element) {
    use dioxus::desktop::Config;
    use dioxus::desktop::WindowBuilder;
    LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_always_on_top(false)))
        .launch(app)
}
