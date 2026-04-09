use noted::errors;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct NoteStore {
    root: PathBuf,
}

impl NoteStore {
    pub fn new(root: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&root)
            .map_err(|e| format!("{}: {}", errors::DAILY_NOTE_CREATE_FAILED, e))?;
        Ok(Self { root })
    }

    pub fn prepare(&self, path: &Path) -> Result<PathBuf, String> {
        let full_path = self.root.join(path);
        fs::create_dir_all(full_path.parent().unwrap())
            .map_err(|e| format!("{}: {}", errors::DAILY_NOTE_CREATE_FAILED, e))?;
        Ok(full_path)
    }

    pub fn append_to_note(
        &self,
        path: &Path,
        text: &str,
        format: impl Fn(&str, &str) -> String,
    ) -> Result<(), String> {
        let full_path = self.prepare(path)?;
        let mut file = fs::OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(full_path)
            .unwrap();
        let mut existing = String::new();
        file.read_to_string(&mut existing).unwrap();
        writeln!(file, "{}", format(&existing, text)).unwrap();
        Ok(())
    }
}
