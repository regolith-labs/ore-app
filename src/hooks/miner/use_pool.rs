use dioxus::prelude::*;

use crate::config::{Pool, FIRST_POOL, SECOND_POOL};

pub fn use_pool_deprecated() -> Resource<Pool> {
    use_resource(move || async move { FIRST_POOL.clone() })
}

pub fn use_pool() -> Resource<Pool> {
    use_resource(move || async move { SECOND_POOL.clone() })
}
