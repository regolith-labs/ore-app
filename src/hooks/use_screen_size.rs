use dioxus::prelude::*;

use crate::{components::Appearance, hooks::use_persistent::use_persistent};

pub enum ScreenSize {
    Desktop,
    Tablet,
    Mobile,
}

pub fn use_screen_size() -> Signal<ScreenSize> {
    let mut screen_size = use_signal(|| ScreenSize::Desktop);

    use_future(move || async move {
        let js_code = r#"
            (function() {
                return window.innerWidth;
            })()
        "#;
        let width = eval(js_code).await;
        if let Ok(width) = width {
            let width = width.as_f64().unwrap_or(0.0);
            let new_screen_size = if width < 768.0 {
                ScreenSize::Mobile
            } else if width < 1024.0 {
                ScreenSize::Tablet
            } else {
                ScreenSize::Desktop
            };
            screen_size.set(new_screen_size);
        }
    });

    screen_size
}
