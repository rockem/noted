#![allow(deprecated)] // cargo_bin deprecation is for custom build-dir, not our use case

use assert_cmd::cargo::CommandCargoExt;
use std::path::{Path, PathBuf};
use std::process::Command;

const _TEST_EDITOR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/test_editor.sh");

#[derive(Debug)]
pub struct AppOutput {
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

    pub fn run(&self) -> Result<AppOutput, String> {
        self.run_with_args(&[])
    }

    pub fn run_with_args(&self, args: &[&str]) -> Result<AppOutput, String> {
        let output = Command::cargo_bin("noted")
            .map_err(|err| format!("Failed to find noted binary: {err}"))?
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", _TEST_EDITOR)
            .args(args)
            .output()
            .map_err(|err| format!("Failed to execute noted: {err}"))?;

        let app_output = AppOutput {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        };

        if output.status.success() {
            Ok(app_output)
        } else {
            Err(app_output.stderr.clone())
        }
    }
}
