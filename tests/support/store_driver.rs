use std::path::PathBuf;
use tempfile::TempDir;

pub struct StoreDriver {
    path: PathBuf,
    _temp_dir: TempDir,
}

impl StoreDriver {
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let path = temp_dir.path().to_path_buf();

        Self {
            path,
            _temp_dir: temp_dir,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn today_note_file_created(&self) {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let note_file = self.path.join(format!("{}.md", today));

        assert!(
            note_file.exists(),
            "Expected daily note file to exist at {:?}",
            note_file
        );
    }
}
