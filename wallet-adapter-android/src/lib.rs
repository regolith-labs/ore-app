// We will use the jni crate to call Kotlin.
use dioxus_mobile;
use jni::objects::{JClass, JObject, JValue};
use jni::signature::ReturnType;
use jni::sys::jvalue; // Import the low-level jvalue type

/// Calls the static `diagnoseClassLoading` method in the Kotlin `KotlinAdder` class via JNI.
///
/// This function obtains the application context by calling
/// `android.app.ActivityThread.currentApplication()`, uses that context to get the application's
/// ClassLoader, and then uses the ClassLoader to load the desired class. If any Java exception occurs
/// (for example, ClassNotFoundException), it is caught (logged and cleared) and an error is returned.
pub fn call_kotlin_add(left: i64, right: i64) -> Result<i64, String> {
    println!(
        "Rust: Attempting to call Kotlin add for {} + {}",
        left, right
    );

    // 1. Obtain the JavaVM (from JNI_OnLoad via dioxus_mobile).
    let vm = dioxus_mobile::get_java_vm().ok_or_else(|| {
        "Failed to get JavaVM instance. JNI_OnLoad might not have run or failed.".to_string()
    })?;

    // Attach the current thread to the JVM and get a JNIEnv.
    let mut env = vm
        .attach_current_thread()
        .map_err(|e| format!("Failed to attach current thread to JVM: {:?}", e))?;

    // 2. Obtain the application context by calling ActivityThread.currentApplication().
    let activity_thread_class = env
        .find_class("android/app/ActivityThread")
        .map_err(|e| format!("Failed to find ActivityThread class: {:?}", e))?;
    let current_app_method_id = env
        .get_static_method_id(
            &activity_thread_class, // Pass a reference instead of the value
            "currentApplication",
            "()Landroid/app/Application;",
        )
        .map_err(|e| format!("Failed to get currentApplication method id: {:?}", e))?;

    // Define the expected return type for currentApplication.
    let ret_app: ReturnType = ReturnType::Object; // Signature is already in get_static_method_id

    let app_obj = unsafe {
        // Pass a reference instead of the value
        env.call_static_method_unchecked(
            &activity_thread_class,
            current_app_method_id,
            ret_app,
            &[],
        )
    }
    .and_then(|v| v.l())
    .map_err(|e| format!("Failed to call currentApplication: {:?}", e))?;
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return Err("Exception occurred when calling currentApplication".to_string());
    }

    // 3. Get the application's ClassLoader via the context's getClassLoader() method.
    let context_class = env
        .find_class("android/content/Context")
        .map_err(|e| format!("Failed to find Context class: {:?}", e))?;
    let get_class_loader_id = env
        .get_method_id(context_class, "getClassLoader", "()Ljava/lang/ClassLoader;")
        .map_err(|e| format!("Failed to get getClassLoader method id: {:?}", e))?;

    let ret_cl: ReturnType = ReturnType::Object; // Signature is already in get_method_id
    let class_loader_obj =
        unsafe { env.call_method_unchecked(app_obj, get_class_loader_id, ret_cl, &[]) }
            .and_then(|v| v.l())
            .map_err(|e| format!("Failed to call getClassLoader: {:?}", e))?;
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return Err("Exception occurred when calling getClassLoader".to_string());
    }

    // 4. Use the ClassLoader to load your Kotlin class.
    let class_loader_class = env
        .find_class("java/lang/ClassLoader")
        .map_err(|e| format!("Failed to find ClassLoader class: {:?}", e))?;
    let load_class_method_id = env
        .get_method_id(
            class_loader_class,
            "loadClass",
            "(Ljava/lang/String;)Ljava/lang/Class;",
        )
        .map_err(|e| format!("Failed to get loadClass method id: {:?}", e))?;
    // Create a Java string with the fully qualified class name.
    let class_name_java = env
        .new_string("ore.supply.KotlinAdder")
        .map_err(|e| format!("Failed to create Java string: {:?}", e))?;

    // Create a JObject from the Java String, binding it to ensure its lifetime.
    let class_name_obj = JObject::from(class_name_java);
    // Create the JValue argument referencing the bound JObject.
    let jvalue_arg = JValue::Object(&class_name_obj);
    // Convert the JValue to the low-level jvalue required by call_method_unchecked.
    let jval_arg: jvalue = jvalue_arg.as_jni(); // Use as_jni() instead of deprecated to_jni()
                                                // Build the arguments array using the jvalue.
    let args = [jval_arg];

    let ret_class: ReturnType = ReturnType::Object; // Signature is already in get_method_id
    let load_class_result = unsafe {
        // Pass the slice of jvalue.
        env.call_method_unchecked(class_loader_obj, load_class_method_id, ret_class, &args)
    };
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return Err(
            "Java exception occurred during loadClass (likely ClassNotFoundException)".to_string(),
        );
    }
    let class_obj = load_class_result.and_then(|v| v.l()).map_err(|e| {
        format!(
            "Failed to load class ore.supply.KotlinAdder via class loader: {:?}",
            e
        )
    })?;

    // 5. Call the static method diagnoseClassLoading on your loaded class.
    let method_name = "diagnoseClassLoading";
    let method_sig = "()V";
    println!(
        "Rust: Calling Kotlin diagnostic method {} with signature {}",
        method_name, method_sig
    );
    let result = env.call_static_method(
        // Convert the loaded class JObject into a JClass.
        JClass::from(class_obj),
        method_name,
        method_sig,
        &[],
    );
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return Err(format!(
            "Java exception occurred during call to {} {}",
            method_name, method_sig
        ));
    }
    result.map_err(|e| {
        format!(
            "Failed to call static method {} {}: {:?}",
            method_name, method_sig, e
        )
    })?;
    println!(
        "Rust: Successfully called Kotlin diagnostic method {}. Check Logcat for output.",
        method_name
    );

    // Return a dummy value, as the called method returns void.
    Ok(0)
}
