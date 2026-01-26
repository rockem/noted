use chrono::Local;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

fn get_store_path() -> PathBuf {
    PathBuf::from(env::var("NOTED_STORE").unwrap())
}

fn get_daily_note_path(store_path: &Path) -> PathBuf {
    let today = Local::now().format("%Y-%m-%d").to_string();
    store_path.join(format!("{}.md", today))
}

fn create_daily_note(path: &PathBuf) -> std::io::Result<File> {
    File::create(path)
}

fn main() {
    let store_path = get_store_path();

    let daily_note_path = get_daily_note_path(&store_path);

    // Only create the daily note if it doesn't already exist
    create_daily_note(&daily_note_path).unwrap();
}
