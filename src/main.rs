use chrono::Local;
use git_version::git_version;
use std::env;
use std::fs::File;
use std::io::Result;
use std::path::{Path, PathBuf};

const _VERSION: &str = git_version!(fallback = "unknown");

fn get_store_path() -> PathBuf {
    env::var("NOTED_STORE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs::data_dir()
                .expect("Could not determine data directory")
                .join("noted")
        })
}

fn get_daily_note_path(store_path: &Path) -> PathBuf {
    let today = Local::now().format("%Y-%m-%d").to_string();
    store_path.join(format!("{}.md", today))
}

fn create_daily_note(path: &PathBuf) -> Result<File> {
    File::create(path)
}

fn main() {
    let store_path = get_store_path();

    std::fs::create_dir_all(&store_path).expect("Failed to create store directory");

    let daily_note_path = get_daily_note_path(&store_path);

    // Only create the daily note if it doesn't already exist
    create_daily_note(&daily_note_path).unwrap();
}
