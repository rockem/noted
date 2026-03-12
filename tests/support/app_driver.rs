#![allow(deprecated)] // cargo_bin deprecation is for custom build-dir, not our use case

use assert_cmd::cargo::CommandCargoExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const TEST_EDITOR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/test_editor.sh");

pub struct AppOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub struct AppDriver {
    store_path: PathBuf,
}

impl AppDriver {
    pub fn new(store_path: &Path) -> Self {
        Self {
            store_path: store_path.to_path_buf(),
        }
    }

    pub fn run(&self) -> AppOutput {
        let output = Command::cargo_bin("noted")
            .expect("Failed to find noted binary")
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", TEST_EDITOR)
            .output()
            .expect("Failed to execute noted");

        AppOutput {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }
}
