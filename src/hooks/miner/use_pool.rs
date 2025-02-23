use dioxus::prelude::*;

use crate::config::{Pool, FIRST_POOL};

pub fn use_pool() -> Resource<Pool> {
    use_resource(move || async move { FIRST_POOL.clone() })
}
