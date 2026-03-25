use chrono::DateTime;
use chrono::Local;
use clap::Parser;
use noted::errors;
use noted::version::VERSION;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_STORE_PATH: &str = "noted";
const DEFAULT_EDITOR: &str = "vim";

#[derive(Parser, Debug)]
#[command(name = "noted", version = VERSION)]
struct Cli {
    #[arg(short = 'e', num_args = 1..)]
    text: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();
    let store_path = get_store_path();
    let daily_note_path = get_daily_note_path(&store_path);

    if let Err(e) = fs::create_dir_all(daily_note_path.parent().unwrap()) {
        eprintln!("{}: {}", errors::DAILY_NOTE_CREATE_FAILED, e);
        std::process::exit(1);
    }

    if let Some(words) = cli.text {
        let text = words.join(" ");
        quick_capture(&daily_note_path, &text);
    } else {
        open_editor(&daily_note_path);
    }
}

fn get_store_path() -> PathBuf {
    env::var("NOTED_STORE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::data_dir().unwrap().join(DEFAULT_STORE_PATH))
}

fn get_daily_note_path(store_path: &Path) -> PathBuf {
    let now = Local::now();
    store_path
        .join("daily")
        .join(now.format("%Y").to_string())
        .join(now.format("%m").to_string())
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

fn quick_capture(path: &Path, text: &str) {
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    writeln!(file).unwrap();
    writeln!(file, "{}", format_entry(Local::now(), "", text)).unwrap();
}

fn format_entry(_time: DateTime<Local>, _existing_text: &str, entry_text: &str) -> String {
    entry_text.to_string()
}

fn open_editor(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    Command::new(&editor).arg(path).status().unwrap();
}
