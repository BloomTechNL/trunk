use std::process::Command;

fn main() {
    // Re-run if HEAD moves (new commit, checkout, etc.)
    println!("cargo:rerun-if-changed=.git/HEAD");
    // Re-run if any ref changes (e.g. branch tip updated after a push)
    println!("cargo:rerun-if-changed=.git/refs/");

    let hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| if o.status.success() { Some(o) } else { None })
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_HASH={hash}");
}

