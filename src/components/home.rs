use dioxus::prelude::*;

use crate::{
    components::{Activity, BackupKeypairWarning, Balance},
    hooks::use_show_backup_warning,
};

#[component]
pub fn Home(cx: Scope) -> Element {
    let show_backup_warning = use_show_backup_warning(cx);
    render! {
        div {
            class: "flex flex-col gap-16 overflow-visible",
            if cfg!(feature = "web") && show_backup_warning.read().0 {
                render! {
                    BackupKeypairWarning {}
                }
            }
            Balance {}
            Activity {}
        }
    }
}
