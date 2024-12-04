use std::process::Command;

fn main() {
    // Get git hash
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    let git_hash = String::from_utf8(output.stdout)
        .expect("Failed to read git hash")
        .trim()
        .to_string();

    // Make git hash available to the build
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
}
