use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct AppOutput {
    pub stdout: String,
    pub stderr: String,
}

pub struct AppDriver {
    store_path: PathBuf,
    editor: String,
}

impl AppDriver {
    pub fn new(store_path: &Path) -> Self {
        Self {
            store_path: store_path.to_path_buf(),
            editor: "true".to_string(),
        }
    }

    pub fn with_editor(mut self, editor: &str) -> Self {
        self.editor = editor.to_string();
        self
    }

    pub fn run(&self) -> Result<AppOutput, String> {
        self.run_with_args(&[])
    }

    pub fn run_with_args(&self, args: &[&str]) -> Result<AppOutput, String> {
        let output = Command::new(assert_cmd::cargo::cargo_bin!("noted"))
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", &self.editor)
            .args(args)
            .output()
            .map_err(|err| format!("Failed to execute noted: {err}"))?;

        Self::to_app_output(output)
    }

    fn to_app_output(output: std::process::Output) -> Result<AppOutput, String> {
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

    pub fn run_with_stdin(&self, args: &[&str], stdin: &str) -> Result<AppOutput, String> {
        let mut child = Command::new(assert_cmd::cargo::cargo_bin!("noted"))
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", &self.editor)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|err| format!("Failed to execute noted: {err}"))?;

        child.stdin.take().unwrap().write_all(stdin.as_bytes()).ok();

        Self::to_app_output(child.wait_with_output().map_err(|e| e.to_string())?)
    }
}
