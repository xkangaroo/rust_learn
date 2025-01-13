use std::process::Command;

fn get_git_info() -> (String, String) {
    // 获取当前分支
    let branch_output = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .expect("Failed to get current branch");

    let branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();

    // 获取当前提交哈希
    let commit_output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .expect("Failed to get current commit hash");

    let commit_hash = String::from_utf8_lossy(&commit_output.stdout).trim().to_string();

    (branch, commit_hash)
}