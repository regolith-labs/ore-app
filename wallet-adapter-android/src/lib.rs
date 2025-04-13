// We will use the jni crate to call Kotlin
use dioxus_mobile;
use jni::objects::JValue;
use jni::sys::jlong; // jlong corresponds to Java's long / Kotlin's Long
                     // JNIEnv is obtained via the JavaVM instance

/// Calls the static `add` method in the Kotlin `KotlinAdder` class via JNI.
pub fn call_kotlin_add(left: i64, right: i64) -> Result<i64, String> {
    println!(
        "Rust: Attempting to call Kotlin add for {} + {}",
        left, right
    );

    // 1. Get the JNI Environment via the JavaVM obtained during JNI_OnLoad
    let vm = match dioxus_mobile::get_java_vm() {
        Some(vm) => vm,
        None => {
            return Err(
                "Failed to get JavaVM instance. JNI_OnLoad might not have run or failed."
                    .to_string(),
            )
        }
    };

    // Attach the current thread to the JVM and get the JNIEnv.
    // This is necessary for any thread that wants to interact with Java objects.
    // The resulting JNIEnv is valid only for the current thread and for the duration
    // of the closure or until detach is called. Using `attach_current_thread`
    // ensures the thread is detached automatically when the `env` guard goes out of scope.
    let mut env = match vm.attach_current_thread() {
        Ok(env) => env,
        Err(e) => return Err(format!("Failed to attach current thread to JVM: {:?}", e)),
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
