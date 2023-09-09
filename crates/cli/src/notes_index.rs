use std::collections::HashMap;
use std::fs::read_dir;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use notes::Note;

/// `Cargo.toml` relative path to the directory containing notes
const MDSVEX_DIR: &str = "src/mdsvex";

#[derive(Debug, Serialize, Deserialize)]
pub struct NotesIndex(HashMap<String, Note>);

impl NotesIndex {
    pub fn new() -> Result<Self> {
        let index = HashMap::new();
        let entries = Self::list_entries()?;

        tracing::info!("Found {} entries", entries.len());

        Ok(Self(index))
    }

    /// Lists entries for notes localted at [`MDSVEX_DIR`]
    fn list_entries() -> Result<Vec<String>> {
        let dir = read_dir(MDSVEX_DIR)?;

        Ok(dir
            .into_iter()
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect())
    }
}
