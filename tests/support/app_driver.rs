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

    pub fn run(&self) {
        let status = Command::cargo_bin("noted")
            .expect("Failed to find noted binary")
            .env("NOTED_STORE", &self.store_path)
            .status()
            .expect("Failed to execute noted");

        assert!(status.success(), "noted command should succeed");
    }
}
