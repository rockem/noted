use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use portable_pty::{CommandBuilder, PtySize, native_pty_system};

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

    /// Runs the command inside a PTY so `stdin().is_terminal()` returns `true` in the subprocess.
    /// Because PTY merges stdout and stderr, both streams appear in `AppOutput.stdout`;
    /// `AppOutput.stderr` will be empty. Use this only when terminal detection is required.
    pub fn run_in_pty(&self, args: &[&str]) -> Result<AppOutput, AppOutput> {
        let mut cmd = CommandBuilder::new(assert_cmd::cargo::cargo_bin!("noted"));
        cmd.env("NOTED_STORE", &self.store_path);
        cmd.env("EDITOR", &self.editor);
        for arg in args {
            cmd.arg(arg);
        }
        let (merged, success) = spawn_in_pty(cmd).expect("Failed to spawn noted in PTY");
        let app_output = AppOutput {
            stdout: merged,
            stderr: String::new(),
        };
        if success {
            Ok(app_output)
        } else {
            Err(app_output)
        }
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
}
/// Spawns `cmd` inside a pseudo-terminal (PTY) so the child process sees a real
/// terminal on its stdin. This ensures `stdin().is_terminal()` returns `true`,
/// matching interactive use.
///
/// Stdout and stderr are merged into the PTY master stream and returned as a
/// single string. The boolean in the `Ok` tuple is `true` when the process
/// exits successfully.
///
/// Note: on macOS, `read_to_string` may return an `EIO` error when the slave
/// side closes after the child exits — this is suppressed via `.ok()` and the
/// collected output up to that point is complete.
fn spawn_in_pty(cmd: CommandBuilder) -> Result<(String, bool), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {e}"))?;

    let mut child = pair
        .slave
        .spawn_command(cmd)
        .map_err(|e| format!("Failed to spawn noted: {e}"))?;

    drop(pair.slave);

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|e| format!("Failed to clone PTY reader: {e}"))?;

    let mut output = String::new();
    reader.read_to_string(&mut output).ok();
    let status = child.wait().map_err(|e| format!("Failed to wait: {e}"))?;
    Ok((output, status.success()))
}
