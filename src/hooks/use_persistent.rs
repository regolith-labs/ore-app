use dioxus::prelude::*;
#[cfg(feature = "web")]
use gloo_storage::{LocalStorage, Storage};
use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "desktop")]
use crate::file::{get_value, set_key_value};

// TODO Wrap this with a useState so all writes auto-update throughout the app

/// A persistent storage hook that can be used to store data across application reloads.
#[allow(clippy::needless_return)]
pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    cx: &ScopeState,
    // A unique key for the storage entry
    key: impl ToString,
    // A function that returns the initial value if the storage entry is empty
    init: impl FnOnce() -> T,
) -> &UsePersistent<T> {
    // Use the use_ref hook to create a mutable state for the storage entry
    let state = use_ref(cx, move || {
        // This closure will run when the hook is created
        let key = key.to_string();

        #[cfg(feature = "web")]
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(|| {
            let value = init();
            LocalStorage::set(key.as_str(), &value).ok();
            value
        });

        #[cfg(feature = "desktop")]
        let value = {
            get_value(key.as_str()).ok().unwrap_or_else(|| {
                let value = init();
                if let Ok(v) = serde_json::to_value(&value) {
                    set_key_value(key.as_str(), &v).ok();
                }
                value
            })
        };

        StorageEntry { key, value }
    });

    // Wrap the state in a new struct with a custom API
    // Note: We use use_hook here so that this hook is easier to use in closures in the rsx. Any values with the same lifetime as the ScopeState can be used in the closure without cloning.
    cx.use_hook(|| UsePersistent {
        inner: state.clone(),
    })
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

/// Storage that persists across application reloads
pub struct UsePersistent<T: 'static> {
    inner: UseRef<StorageEntry<T>>,
}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    /// Returns a reference to the value
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    /// Sets the value
    pub fn set(&self, value: T) {
        let mut inner = self.inner.write();

        // Write the new value to local storage
        #[cfg(feature = "web")]
        LocalStorage::set(inner.key.as_str(), &value).unwrap();

        // TODO Handle desktop
        #[cfg(feature = "desktop")]
        {
            set_key_value(inner.key.as_str(), &value).unwrap();
        }

        inner.value = value;
    }
}
