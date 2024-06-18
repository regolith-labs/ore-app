use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ShowBackupWarning(pub bool);

const KEY: &str = "show_backup_warning";

pub fn use_show_backup_warning() -> Signal<ShowBackupWarning> {
    let show_backup_warning = use_context::<Signal<ShowBackupWarning>>();
    let mut show_backup_warning_persistent = use_persistent(KEY, || ShowBackupWarning(true));
    use_effect(move || show_backup_warning_persistent.set(*show_backup_warning.read()));
    show_backup_warning
}

pub fn use_show_backup_warning_provider() {
    let show_backup_warning = use_persistent(KEY, || ShowBackupWarning(true)).get();
    use_context_provider(|| Signal::new(show_backup_warning));
}
