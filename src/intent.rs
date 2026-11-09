use crate::{cli::RiskLevel, worktree::git};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    process,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IntentMetadata {
    pub agent: String,
    pub intent: String,
    pub scope: Vec<String>,
    pub risk: Option<RiskLevel>,
}

impl IntentMetadata {
    fn git_path(worktree: &Path) -> Result<PathBuf> {
        let git_dir = git(worktree, &["rev-parse", "--absolute-git-dir"])?;
        Ok(Path::new(git_dir.trim_end_matches(['\r', '\n'])).join("prysm-intent.json"))
    }

    pub fn save(&self, worktree: &Path) -> Result<()> {
        let path = Self::git_path(worktree)?;
        let pending = path.with_extension(format!("{}.tmp", process::id()));
        let content = serde_json::to_vec_pretty(self)?;
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&pending)?;
        let written = file.write_all(&content);
        drop(file);
        let result = written.and_then(|()| fs::rename(&pending, &path));
        if result.is_err() {
            let _ = fs::remove_file(&pending);
        }
        result.with_context(|| format!("Could not save intent to {}", path.display()))
    }

    pub fn load(worktree: &Path) -> Result<Option<Self>> {
        let current = Self::git_path(worktree)?;
        let legacy = worktree.join(".prysm").join("intent.json");
        for path in [current, legacy] {
            match fs::read(&path) {
                Ok(content) => {
                    return serde_json::from_slice(&content)
                        .map(Some)
                        .with_context(|| format!("Invalid intent metadata at {}", path.display()));
                }
                Err(error) if error.kind() == ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(error).with_context(|| format!("Could not read {}", path.display()))
                }
            }
        }
        Ok(None)
    }
}
