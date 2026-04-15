mod cli;
mod daily;
mod notes;

use chrono::Local;
use clap::Parser;
use cli::{Cli, Commands};
use notes::NoteStore;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_STORE_PATH: &str = "noted";
const DEFAULT_EDITOR: &str = "vim";

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let Commands::Daily(args) = cli.command;

    let store = NoteStore::new(get_store_path())?;
    let daily_path = daily::get_daily_note_path();

    if let Some(words) = args.text {
        let text = daily::resolve_capture_text(words)?;
        let now = Local::now();
        store.append_to_note(&daily_path, &text, |existing, entry| {
            daily::format_entry(now, existing, entry)
        })?;
    } else {
        let full_path = store.prepare(&daily_path)?;
        open_editor(&full_path);
    }
    Ok(())
}

fn get_store_path() -> PathBuf {
    env::var("NOTED_STORE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::data_dir().unwrap().join(DEFAULT_STORE_PATH))
}

fn open_editor(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    Command::new(&editor).arg(path).status().unwrap();
}
