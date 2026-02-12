use std::env;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StoreDriver {
    path: PathBuf,
}

impl StoreDriver {
    pub fn new() -> Self {
        Self {
            path: Self::test_store_path(),
        }
    }

    fn test_store_path() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        env::temp_dir().join(format!("noted-test-{}", unique_id))
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

impl Drop for StoreDriver {
    fn drop(&mut self) {
        if self.path.exists() {
            std::fs::remove_dir_all(&self.path).ok();
        }
    }
}
