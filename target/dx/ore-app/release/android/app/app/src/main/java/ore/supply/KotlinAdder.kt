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
        }
    }
}
