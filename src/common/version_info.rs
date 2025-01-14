// src/common/version_info.rs
use std::process::Command;
use chrono::{Local, DateTime};

pub struct VersionInfo {
    pub branch: String,
    pub commit_hash: String,
    pub build_time: DateTime<Local>,
}

impl VersionInfo {
    pub fn new(branch: String, commit_hash: String) -> Self {
        let build_time: DateTime<Local> = Local::now();
        VersionInfo { branch, commit_hash, build_time }
    }

    pub fn print(&self) {
        println!("当前分支: {}", self.branch);
        println!("提交哈希: {}", self.commit_hash);
        println!("构建时间: {}", self.build_time);
    }
}

pub trait VersionInfoProvider {
    fn get_version_info(&self) -> Result<VersionInfo, String>;
}

pub struct GitVersionInfoProvider;

impl VersionInfoProvider for GitVersionInfoProvider {
    fn get_version_info(&self) -> Result<VersionInfo, String> {
        let branch = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .map_err(|e| format!("Failed to execute git command: {}", e))?
            .stdout;

        let branch = String::from_utf8(branch).map_err(|e| format!("Invalid utf8 string: {}", e))?.trim().to_string();

        let commit_hash = Command::new("git")
            .arg("rev-parse")
            .arg("HEAD")
            .output()
            .map_err(|e| format!("Failed to execute git command: {}", e))?
            .stdout;

        let commit_hash = String::from_utf8(commit_hash).map_err(|e| format!("Invalid utf8 string: {}", e))?.trim().to_string();

        Ok(VersionInfo::new(branch, commit_hash))
    }
}
