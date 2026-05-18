use std::process::Command;

fn main() {
    // The release workflow sets CRYO_VERSION explicitly when building the
    // Docker image (its build context has no .git). Otherwise derive the
    // version from git, then fall back to the Cargo.toml version.
    println!("cargo:rerun-if-env-changed=CRYO_VERSION");
    let git_description = std::env::var("CRYO_VERSION")
        .ok()
        .filter(|v| !v.is_empty())
        .or_else(|| get_git_description().ok())
        .unwrap_or_else(|| env!("CARGO_PKG_VERSION").to_string());

    println!("cargo:rustc-env=GIT_DESCRIPTION={}", git_description);
}

fn get_git_description() -> Result<String, std::io::Error> {
    let output = Command::new("git").args(["describe", "--tags", "--always"]).output()?;

    if output.status.success() {
        let description = String::from_utf8(output.stdout)
            .expect("Failed to read git command output")
            .trim()
            .to_string();

        Ok(description)
    } else {
        Err(std::io::Error::other("Git command failed"))
    }
}
