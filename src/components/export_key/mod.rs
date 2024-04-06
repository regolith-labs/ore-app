mod export_key_secret;
mod export_key_warning;

use dioxus::prelude::*;
pub use export_key_secret::*;
pub use export_key_warning::*;

pub enum ExportKeyStep {
    Warning,
    Secret,
}

#[component]
pub fn ExportKey(cx: Scope) -> Element {
    let step = use_state(cx, || ExportKeyStep::Warning);

    match step.get() {
        ExportKeyStep::Warning => {
            render! {
                ExportKeyWarning { step: step }
            }
        }
        ExportKeyStep::Secret => {
            render! {
                ExportKeySecret {}
            }
        }
    }
}
