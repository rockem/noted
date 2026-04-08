use chrono::DateTime;
use chrono::Local;
use clap::{Parser, Subcommand};
use noted::errors;
use noted::version::VERSION;
use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_STORE_PATH: &str = "noted";
const DEFAULT_EDITOR: &str = "vim";

#[derive(Parser, Debug)]
#[command(name = "noted", version = VERSION)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(alias = "d", about = "Create or edit today's daily note")]
    Daily(DailyArgs),
}

#[derive(Parser, Debug)]
struct DailyArgs {
    #[arg(short = 'e', num_args = 0.., help = "Quick capture text. Use '-' to read from stdin.")]
    text: Option<Vec<String>>,
}

fn main() {
    let cli = Cli::parse();
    let Commands::Daily(args) = cli.command;

    let store_path = get_store_path();
    let daily_note_path = get_daily_note_path(&store_path);

    if let Err(e) = fs::create_dir_all(daily_note_path.parent().unwrap()) {
        eprintln!("{}: {}", errors::DAILY_NOTE_CREATE_FAILED, e);
        std::process::exit(1);
    }

    if let Some(words) = args.text {
        match resolve_capture_text(words) {
            Ok(text) => quick_capture(&daily_note_path, &text),
            Err(msg) => {
                eprintln!("{msg}");
                std::process::exit(1);
            }
        }
    } else {
        open_editor(&daily_note_path);
    }
}

fn resolve_capture_text(words: Vec<String>) -> Result<String, String> {
    match words.as_slice() {
        [] => Err("-e flag requires text or \"-\" for stdin".to_string()),
        [dash] if dash == "-" => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            Ok(s.trim().to_string())
        }
        _ => Ok(words.join(" ")),
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
        .read(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut existing = String::new();
    file.read_to_string(&mut existing).unwrap();
    writeln!(file, "{}", format_entry(Local::now(), &existing, text)).unwrap();
}

fn format_entry(time: DateTime<Local>, existing_text: &str, entry_text: &str) -> String {
    let prefix = if existing_text.is_empty() || existing_text.ends_with('\n') {
        ""
    } else {
        "\n"
    };
    format!("{}**{}** {}", prefix, time.format("%H:%M"), entry_text)
}

fn open_editor(path: &Path) {
    let editor = env::var("EDITOR").unwrap_or_else(|_| DEFAULT_EDITOR.to_string());
    Command::new(&editor).arg(path).status().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn format_entry_prepends_bold_time() {
        verify_entry_format("", "some text", "**14:30** some text");
    }

    fn verify_entry_format(existing_text: &str, entry_text: &str, expected_text: &str) {
        let fixed_time = Local.with_ymd_and_hms(2026, 3, 26, 14, 30, 0).unwrap();
        assert_eq!(
            format_entry(fixed_time, existing_text, entry_text),
            expected_text
        );
    }

    #[test]
    fn format_entry_adds_newline_when_existing_content() {
        verify_entry_format("existing content", "some text", "\n**14:30** some text")
    }

    #[test]
    fn format_entry_no_extra_newline_when_existing_ends_with_newline() {
        verify_entry_format("existing content\n", "some text", "**14:30** some text")
    }

    #[test]
    fn e_flag_with_no_text_returns_error() {
        assert!(resolve_capture_text(vec![]).is_err());
    }

    #[test]
    fn e_flag_with_text_returns_text() {
        assert_eq!(resolve_capture_text(vec!["hello".into()]).unwrap(), "hello");
    }
}
