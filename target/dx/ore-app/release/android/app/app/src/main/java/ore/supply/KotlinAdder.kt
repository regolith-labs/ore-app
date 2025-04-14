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
            Log.d("KotlinAdder", "--- Starting Class Loading Diagnosis ---")

            // Get the ClassLoader associated with this class
            val classLoader = KotlinAdder::class.java.classLoader
            if (classLoader != null) {
                Log.d("KotlinAdder", "Using ClassLoader: $classLoader")

                // Attempt 1: Load using the specific ClassLoader instance
                try {
                    classLoader.loadClass(className)
                    Log.d("KotlinAdder", "SUCCESS (Attempt 1): Class '$className' found using classLoader.loadClass().")
                } catch (e: ClassNotFoundException) {
                    Log.e(
                        "KotlinAdder",
                        "FAILURE (Attempt 1): Class '$className' NOT found using classLoader.loadClass(). Error: ${e.message}",
                    )
                } catch (t: Throwable) {
                    Log.e(
                        "KotlinAdder",
                        "ERROR (Attempt 1): Unexpected error using classLoader.loadClass() for '$className'. Error: ${t.message}",
                    )
                }
            } else {
                Log.e("KotlinAdder", "ERROR: Could not get ClassLoader for KotlinAdder::class.java.")
            }

            // Attempt 2: Load using Class.forName (uses the caller's ClassLoader implicitly)
            try {
                Class.forName(className)
                Log.d("KotlinAdder", "SUCCESS (Attempt 2): Class '$className' found using Class.forName().")
            } catch (e: ClassNotFoundException) {
                Log.e("KotlinAdder", "FAILURE (Attempt 2): Class '$className' NOT found using Class.forName(). Error: ${e.message}")
            } catch (t: Throwable) {
                Log.e("KotlinAdder", "ERROR (Attempt 2): Unexpected error using Class.forName() for '$className'. Error: ${t.message}")
            }
            Log.d("KotlinAdder", "--- Finished Class Loading Diagnosis ---")
        }
    }
}
