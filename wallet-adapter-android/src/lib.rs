// We will use the jni crate to call Kotlin
use jni::objects::{JClass, JValue};
use jni::sys::jlong;
use jni::JNIEnv; // jlong corresponds to Java's long / Kotlin's Long

// *** Critical Placeholder ***
// We need a way to get the JNIEnv. In a typical Android app, this is passed
// to native functions (e.g., JNI_OnLoad). In Dioxus/cargo-mobile, we need to
// find how it's exposed. This might involve using `android_activity` or
// specific hooks provided by the framework.
// This function signature is a placeholder!
fn get_jni_env() -> Option<JNIEnv<'static>> {
    // Placeholder: In a real scenario, you'd get this from the Android environment.
    // This might involve unsafe code or specific framework functions.
    // For example, using android_activity::AndroidApp::jni_env() if available.
    // Returning None here will cause the call to fail safely in this example.
    // You MUST replace this with the actual way to get the JNIEnv in your Dioxus setup.
    eprintln!("WARNING: get_jni_env() is a placeholder and needs implementation!");

    // Example using android_activity (if integrated):
    /*
    if let Some(app) = &*APP_HANDLE.lock().unwrap() { // Assuming APP_HANDLE stores android_activity::AndroidApp
         return Some(app.jni_env());
    }
    */

    None
    // A common but potentially unsafe way if you *know* the current thread is attached:
    /*
    use jni::JavaVM;
    use std::sync::Arc;
    // Assume JVM: Arc<JavaVM> is stored globally somewhere after JNI_OnLoad
    match JVM.get_env() {
        Ok(env) => Some(env),
        Err(_) => {
            eprintln!("Failed to get JNIEnv: Thread not attached?");
            None
        }
    }
    */
}

/// Calls the static `add` method in the Kotlin `KotlinAdder` class via JNI.
pub fn call_kotlin_add(left: i64, right: i64) -> Result<i64, String> {
    println!(
        "Rust: Attempting to call Kotlin add for {} + {}",
        left, right
    );

    // 1. Get the JNI Environment
    // SAFETY: Getting the JNIEnv might require unsafe blocks depending on the method.
    // The lifetime 'static is often used here but needs careful consideration
    // based on how the JNIEnv is obtained and managed.
    let env = match get_jni_env() {
        Some(env) => env,
        None => return Err("Failed to get JNIEnv".to_string()),
    };

    // 2. Find the Kotlin class
    // Note: Use '/' instead of '.' for package names in JNI.
    // Use the package name from build.gradle.kts
    let class_name = "ore/supply/KotlinAdder";
    let class = match env.find_class(class_name) {
        Ok(c) => c,
        Err(e) => return Err(format!("Failed to find class {}: {:?}", class_name, e)),
    };

    // 3. Call the static method
    // Method name: "add"
    // Signature: "(JJ)J" means it takes two longs (J) and returns a long (J).
    // See JNI type signatures documentation.
    let method_name = "add";
    let method_sig = "(JJ)J";

    let j_left = JValue::from(left as jlong);
    let j_right = JValue::from(right as jlong);

    match env.call_static_method(class, method_name, method_sig, &[j_left, j_right]) {
        Ok(result) => {
            // 4. Convert the result
            match result.j() {
                // .j() attempts to convert JValue to jlong
                Ok(value) => {
                    println!("Rust: Received result from Kotlin: {}", value);
                    Ok(value)
                }
                Err(e) => Err(format!("Failed to convert Kotlin result to long: {:?}", e)),
            }
        }
        Err(e) => Err(format!(
            "Failed to call static method {} {}: {:?}",
            method_name, method_sig, e
        )),
    }
}

// Example of how you might call this from your Dioxus component:
/*
use dioxus::prelude::*;

fn App(cx: Scope) -> Element {
    let result_state = use_state(cx, || "Click to call Kotlin".to_string());

    cx.render(rsx! {
        div {
            button {
                onclick: move |_| {
                    match call_kotlin_add(25, 17) {
                        Ok(result) => result_state.set(format!("Kotlin result: {}", result)),
                        Err(e) => result_state.set(format!("Error: {}", e)),
                    }
                },
                "{result_state}"
            }
        }
    })
}
*/
