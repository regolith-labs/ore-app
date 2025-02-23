use dioxus::prelude::*;

use crate::hooks::use_pool;

pub fn use_pool_url() -> Memo<Option<String>> {
    let pool = use_pool();
    use_memo(move || {
        let Some(pool) = pool.cloned() else {
            return None;
        };
        Some(pool.url)
    })
}
