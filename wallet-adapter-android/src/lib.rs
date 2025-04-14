// We will use the jni crate to call Kotlin
use dioxus_mobile;
use jni::objects::JValue;
use jni::sys::jlong; // jlong corresponds to Java's long / Kotlin's Long

/// Calls the static `diagnoseClassLoading` method in the Kotlin `KotlinAdder` class via JNI.
/// Note: This function returns a dummy value (0) on success because the method returns void.
pub fn call_kotlin_add(left: i64, right: i64) -> Result<i64, String> {
    println!(
        "Rust: Attempting to call Kotlin add for {} + {}",
        left, right
    );

    // 1. Get the JNI Environment via the JavaVM obtained during JNI_OnLoad
    let vm = dioxus_mobile::get_java_vm().ok_or_else(|| {
        "Failed to get JavaVM instance. JNI_OnLoad might not have run or failed.".to_string()
    })?;

    // Attach the current thread to the JVM and get the JNIEnv.
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("Failed to attach current thread to JVM: {:?}", e))?;

    // 2. Find the Kotlin class.
    // Note: Use '/' instead of '.' for package names in JNI.
    // Ensure the fully qualified class name exactly matches your Kotlin declaration.
    let class_name = "ore/supply/KotlinAdder";
    let class = env
        .find_class(class_name)
        .map_err(|e| format!("Failed to find class {}: {:?}", class_name, e))?;

    // 3. Set up the static method to call.
    // Method: diagnoseClassLoading()
    // Signature: "()V" means it takes no arguments and returns void.
    let method_name = "diagnoseClassLoading";
    let method_sig = "()V";

    println!(
        "Rust: Calling Kotlin diagnostic method {} with signature {}",
        method_name, method_sig
    );

    // 4. Call the static method
    let result = env.call_static_method(class, method_name, method_sig, &[]);

    // Check if the call itself failed.
    if let Err(e) = result {
        return Err(format!(
            "Failed to call static method {} {}: {:?}",
            method_name, method_sig, e
        ));
    }

    // 5. Immediately check for a pending Java exception.
    if let Ok(true) = env.exception_check() {
        // Optionally, log the exception details to Logcat.
        let _ = env.exception_describe();
        // Clear the exception to reset the JNI state.
        let _ = env.exception_clear();
        return Err(format!(
            "Java exception occurred during call to {} {}",
            method_name, method_sig
        ));
    }

    println!(
        "Rust: Successfully called Kotlin diagnostic method {}. Check Logcat for output.",
        method_name
    );

    // As the called method returns void, we return a dummy value.
    Ok(0)
}
