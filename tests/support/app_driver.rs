#![allow(deprecated)] // cargo_bin deprecation is for custom build-dir, not our use case

use assert_cmd::cargo::CommandCargoExt;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct AppDriver {
    store_path: PathBuf,
}

impl AppDriver {
    pub fn new(store_path: &Path) -> Self {
        Self {
            store_path: store_path.to_path_buf(),
        }
    }

    pub fn run(&self) -> String {
        let output = Command::cargo_bin("noted")
            .expect("Failed to find noted binary")
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", "cat")
            .output()
            .expect("Failed to execute noted");

        assert!(
            output.status.success(),
            "noted command should succeed\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}
