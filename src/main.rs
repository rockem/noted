use chrono::Local;
use git_version::git_version;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const _VERSION: &str = git_version!(fallback = "unknown");

const _DEFAULT_STORE_PATH: &str = "noted";
const _DEFAULT_EDITOR: &str = "vim";

fn main() {
    let store_path = get_store_path();
    let daily_note_path = get_daily_note_path(&store_path);

    if let Err(e) = fs::create_dir_all(daily_note_path.parent().unwrap()) {
        eprintln!("Error: Failed to create daily note: {}", e);
        std::process::exit(1);
    }

    open_editor(&daily_note_path);
}

fn get_store_path() -> PathBuf {
    env::var("NOTED_STORE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::data_dir().unwrap().join(_DEFAULT_STORE_PATH))
}

fn get_daily_note_path(store_path: &Path) -> PathBuf {
    let now = Local::now();
    store_path
        .join("daily")
        .join(now.format("%Y").to_string())
        .join(now.format("%m").to_string())
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

fn open_editor(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| _DEFAULT_EDITOR.to_string());
    Command::new(&editor).arg(path).status().unwrap();
}
