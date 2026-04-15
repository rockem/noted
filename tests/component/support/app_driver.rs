use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

#[derive(Debug)]
pub struct AppOutput {
    pub stdout: String,
    pub stderr: String,
}

impl AppOutput {
    fn from(output: Output) -> Result<AppOutput, AppOutput> {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if output.status.success() {
            Ok(AppOutput { stdout, stderr })
        } else {
            Err(AppOutput { stdout, stderr })
        }
    }
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

    #[allow(dead_code)]
    pub fn run(&self) -> Result<AppOutput, AppOutput> {
        self.run_with_args(&[])
    }

    pub fn run_with_args(&self, args: &[&str]) -> Result<AppOutput, AppOutput> {
        let output = self
            .create_command(args, Stdio::null())
            .output()
            .expect("Failed to spawn noted");
        AppOutput::from(output)
    }

    pub fn run_with_stdin(&self, args: &[&str], stdin: &str) -> Result<AppOutput, AppOutput> {
        let mut child = self
            .create_command(args, Stdio::piped())
            .spawn()
            .expect("Failed to spawn noted");

        child.stdin.take().unwrap().write_all(stdin.as_bytes()).ok();

        let output = child.wait_with_output().expect("Failed to wait for noted");
        AppOutput::from(output)
    }

    fn create_command(&self, args: &[&str], stdin: Stdio) -> Command {
        let mut command = Command::new(assert_cmd::cargo::cargo_bin!("noted"));
        command
            .env("NOTED_STORE", &self.store_path)
            .env("EDITOR", &self.editor)
            .args(args)
            .stdin(stdin)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        command
    }
}
