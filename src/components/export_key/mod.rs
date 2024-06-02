mod secret;
mod warning;

pub use secret::*;
pub use warning::*;

use dioxus::prelude::*;

pub enum ExportKeyStep {
    Warning,
    Secret,
}

pub fn ExportKey() -> Element {
    let step = use_signal(|| ExportKeyStep::Warning);

    let e = match *step.read() {
        ExportKeyStep::Warning => {
            rsx! {
                ExportKeyWarning { step }
            }
        }
        ExportKeyStep::Secret => {
            rsx! {
                ExportKeySecret {}
            }
        }
    };

    e
}
