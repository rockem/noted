use chrono::NaiveDate;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

fn set_readonly(path: &Path, readonly: bool) -> std::io::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(readonly);
    fs::set_permissions(path, perms)
}

pub struct StoreDriver {
    path: PathBuf,
    today: NaiveDate,
}

impl StoreDriver {
    pub fn new() -> Self {
        Self {
            path: Self::test_store_path(),
            today: chrono::Local::now().date_naive(),
        }
    }

    pub fn new_with_create() -> Self {
        let path = Self::test_store_path();
        std::fs::create_dir_all(&path).unwrap();
        Self {
            path,
            today: chrono::Local::now().date_naive(),
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

    pub fn create_today_note(&self, content: &str) -> PathBuf {
        let path = self.today_note_path();
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, content).unwrap();
        path
    }

    pub fn today_note_path(&self) -> PathBuf {
        self.path
            .join("daily")
            .join(self.today.format("%Y").to_string())
            .join(self.today.format("%m").to_string())
            .join(format!("{}.md", self.today.format("%Y-%m-%d")))
    }

    pub fn make_read_only(&self) {
        let _ = set_readonly(&self.path, true);
    }

    pub fn today_note_file_created(&self) {
        let note_file = self.today_note_path();
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
            let _ = set_readonly(&self.path, false);
            std::fs::remove_dir_all(&self.path).ok();
        }
    }
}
