package ore.supply // Using the namespace from build.gradle.kts

import android.util.Log

// Standard Kotlin class, not implementing any UniFFI interface.
public class KotlinAdder {
    // Simple add method using Long (Java's long) for easier JNI interop.
    // This method needs to be callable from Rust via JNI.
    // Adding @JvmStatic makes it a static method, which can be simpler to call from JNI
    // as you don't need to instantiate the class first.
    companion object {
        @JvmStatic
        fun add(
            left: Long,
            right: Long,
        ): Long {
            Log.d("KotlinAdder", "Kotlin static add method called with: $left + $right")
            val result = left + right
            Log.d("KotlinAdder", "Kotlin static add result: $result")
            return result
            return result
        }

        @JvmStatic
        fun diagnoseClassLoading() {
            val className = "ore.supply.KotlinAdder"
            try {
                // Attempt to load the class using the default class loader
                Class.forName(className)
                Log.d("KotlinAdder", "SUCCESS: Class '$className' found by Class.forName().")
            } catch (e: ClassNotFoundException) {
                Log.e("KotlinAdder", "FAILURE: Class '$className' NOT found by Class.forName(). Error: ${e.message}")
                // Optionally log the stack trace for more details
                // Log.e("KotlinAdder", "Stack trace:", e)
            } catch (t: Throwable) {
                // Catch other potential errors during class loading/initialization
                Log.e("KotlinAdder", "ERROR: An unexpected error occurred while trying to load class '$className'. Error: ${t.message}")
                // Log.e("KotlinAdder", "Stack trace:", t)
            }
        }
    }
}
