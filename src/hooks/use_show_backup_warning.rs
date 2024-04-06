use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::hooks::use_persistent::use_persistent;

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct ShowBackupWarning(pub bool);

const KEY: &str = "show_backup_warning";

pub fn use_show_backup_warning(cx: &ScopeState) -> &UseSharedState<ShowBackupWarning> {
    let show_backup_warning = use_shared_state::<ShowBackupWarning>(cx).unwrap();
    let show_backup_warning_persistent = use_persistent(cx, KEY, || ShowBackupWarning(true));
    use_effect(cx, show_backup_warning, |_| {
        show_backup_warning_persistent.set(*show_backup_warning.read());
        async move {}
    });
    show_backup_warning
}

pub fn use_show_backup_warning_provider(cx: &ScopeState) {
    let show_backup_warning = use_persistent(cx, KEY, || ShowBackupWarning(true)).get();
    use_shared_state_provider(cx, || show_backup_warning);
}
