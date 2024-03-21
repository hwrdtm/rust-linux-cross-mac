fn main() {
    // First get the current git branch.
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to execute git command");
    assert!(output.status.success());

    // Then get the git commit sha of the `test` branch using git ls-remote.
    let output = std::process::Command::new("git")
        .args(&["ls-remote", "origin", "test"])
        .output()
        .expect("Failed to execute git command");
    assert!(output.status.success());
    let commit_sha = String::from_utf8(output.stdout).unwrap();
    let commit_sha = commit_sha.split_whitespace().next().unwrap();

    // Then git checkout that commit sha.
    let output = std::process::Command::new("git")
        .args(&["checkout", commit_sha])
        .output()
        .expect("Failed to execute git command");
    assert!(output.status.success());
        
    // Run `cargo build` in the directory above by first cd'ing to it.
    let output = std::process::Command::new("sh")
        .args(&["-c", "cd .. && cargo build"])
        .output()
        .expect("Failed to execute cargo command");
    assert!(output.status.success());

    // Then, rename the binary ./target/debug/rust-linux-cross-mac to ./target/debug/build_<commit_sha>
    let output = std::process::Command::new("sh")
        .args(&["-c", format!("mv ../target/debug/rust-linux-cross-mac ../target/debug/build_{}", commit_sha).as_str()])
        .output()
        .expect("Failed to execute mv command");
    assert!(output.status.success());
}
