use chrono::{DateTime, Local};
use std::io::Read;
use std::path::PathBuf;

pub fn get_daily_note_path() -> PathBuf {
    let now = Local::now();
    PathBuf::from("daily")
        .join(now.format("%Y").to_string())
        .join(now.format("%m").to_string())
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

pub fn resolve_capture_text(words: Vec<String>) -> Result<String, String> {
    match words.as_slice() {
        [] => Err("-e flag requires text or \"-\" for stdin".to_string()),
        [dash] if dash == "-" => {
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            Ok(s.to_string())
        }
        _ => Ok(words.join(" ")),
    }
}

pub fn format_entry(time: DateTime<Local>, existing_text: &str, entry_text: &str) -> String {
    let prefix = if existing_text.is_empty() || existing_text.ends_with('\n') {
        ""
    } else {
        "\n"
    };
    format!("{}**{}** {}", prefix, time.format("%H:%M"), entry_text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn format_entry_prepends_bold_time() {
        verify_entry_format("", "some text", "**14:30** some text");
    }

    #[test]
    fn format_entry_adds_newline_when_existing_content() {
        verify_entry_format("existing content", "some text", "\n**14:30** some text")
    }

    #[test]
    fn format_entry_no_extra_newline_when_existing_ends_with_newline() {
        verify_entry_format("existing content\n", "some text", "**14:30** some text")
    }

    fn verify_entry_format(existing_text: &str, entry_text: &str, expected_text: &str) {
        let fixed_time = Local.with_ymd_and_hms(2026, 3, 26, 14, 30, 0).unwrap();
        assert_eq!(
            format_entry(fixed_time, existing_text, entry_text),
            expected_text
        );
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
