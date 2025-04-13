// Top-level build file for the Android project.
// This file is typically located at the root of the Gradle project.
// Here, we define plugin versions used across modules.
plugins {
    // Define the Android Gradle Plugin version
    // 'apply false' means the plugin is not applied to the root project itself,
    // but makes the version available to subprojects (like :app).
    id("com.android.application") version "8.2.2" apply false

    // Define the Kotlin Android plugin version
    id("org.jetbrains.kotlin.android") version "1.9.0" apply false
}

// Task to clean the build directory (optional but common)
tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}
