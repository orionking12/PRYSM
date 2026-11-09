use crate::{cli::RiskLevel, intent::IntentMetadata};
use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

#[derive(Default, Serialize)]
pub(crate) struct Worktree {
    pub path: PathBuf,
    pub head: String,
    pub branch: Option<String>,
    pub bare: bool,
    pub locked: bool,
    pub prunable: bool,
}

pub(crate) fn git(dir: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .output()
        .context("Could not run Git. Check that Git is on PATH")?;
    if !output.status.success() {
        bail!(
            "git {} failed ({}): {}",
            args.join(" "),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }
    String::from_utf8(output.stdout).context("Git returned output that is not UTF-8")
}

pub(crate) fn worktrees() -> Result<Vec<Worktree>> {
    let output = git(Path::new("."), &["worktree", "list", "--porcelain", "-z"])?;
    let mut trees = Vec::new();
    let mut tree = Worktree::default();
    for field in output.split('\0') {
        let (key, value) = field.split_once(' ').unwrap_or((field, ""));
        match key {
            "worktree" => tree.path = PathBuf::from(value),
            "HEAD" => tree.head = value.to_owned(),
            "branch" => tree.branch = value.strip_prefix("refs/heads/").map(str::to_owned),
            "bare" => tree.bare = true,
            "locked" => tree.locked = true,
            "prunable" => tree.prunable = true,
            "" if !tree.path.as_os_str().is_empty() => trees.push(std::mem::take(&mut tree)),
            _ => {}
        }
    }
    Ok(trees)
}

pub(crate) fn root(dir: &Path) -> Result<PathBuf> {
    let output = git(dir, &["rev-parse", "--show-toplevel"])?;
    Ok(PathBuf::from(output.trim_end_matches(['\r', '\n'])))
}

pub(crate) fn branch_ref(branch: &str) -> Result<String> {
    let checked = git(Path::new("."), &["check-ref-format", "--branch", branch])?;
    if checked.trim_end() != branch {
        bail!("Use a literal branch name");
    }
    Ok(format!("refs/heads/{branch}"))
}

pub(crate) fn current(trees: &[Worktree]) -> Result<&Worktree> {
    let here = root(Path::new("."))?.canonicalize()?;
    trees
        .iter()
        .find(|tree| tree.path.canonicalize().is_ok_and(|path| path == here))
        .context("Current Git worktree is missing from the worktree list")
}

pub(crate) fn path_arg(path: &Path) -> Result<&str> {
    path.to_str().context("Git path is not valid UTF-8")
}

pub fn switch(
    branch: String,
    create: bool,
    agent: Option<String>,
    intent: Option<String>,
    scope: Vec<String>,
    exec: Option<String>,
    risk: Option<RiskLevel>,
) -> Result<()> {
    let reference = branch_ref(&branch)?;
    let trees = worktrees()?;
    let here = current(&trees)?;
    let existing = trees
        .iter()
        .find(|tree| tree.branch.as_deref() == Some(branch.as_str()));
    let path = if let Some(tree) = existing {
        if create {
            bail!(
                "Branch {branch} already has a worktree at {}",
                tree.path.display()
            );
        }
        if tree.prunable || !tree.path.is_dir() {
            bail!("Worktree path is unavailable: {}", tree.path.display());
        }
        tree.path.clone()
    } else {
        let primary = &trees.first().context("No Git worktrees found")?.path;
        let parent = primary
            .parent()
            .context("Primary worktree has no parent directory")?;
        let name = primary
            .file_name()
            .context("Primary worktree has no directory name")?;
        let path = parent.join(".prysm_worktrees").join(name).join(&branch);
        let path_str = path_arg(&path)?;
        if create {
            git(
                &here.path,
                &["worktree", "add", "-b", &branch, "--", path_str],
            )?;
        } else {
            git(&here.path, &["show-ref", "--verify", "--", &reference])?;
            git(&here.path, &["worktree", "add", "--", path_str, &branch])?;
        }
        path
    };

    let previous = IntentMetadata::load(&path)?;
    if previous.is_none()
        || agent.is_some()
        || intent.is_some()
        || !scope.is_empty()
        || risk.is_some()
    {
        let mut metadata = previous.unwrap_or_else(|| IntentMetadata {
            agent: "human".to_owned(),
            intent: "General development".to_owned(),
            scope: Vec::new(),
            risk: None,
        });
        if let Some(agent) = agent {
            metadata.agent = agent;
        }
        if let Some(intent) = intent {
            metadata.intent = intent;
        }
        if !scope.is_empty() {
            metadata.scope = scope;
        }
        if risk.is_some() {
            metadata.risk = risk;
        }
        metadata.save(&path)?;
    }

    println!("Worktree: {}", path.display());
    if let Some(command) = exec {
        let mut shell = if cfg!(windows) {
            let mut shell =
                Command::new(env::var_os("ComSpec").unwrap_or_else(|| "cmd.exe".into()));
            shell.arg("/C");
            shell
        } else {
            let mut shell = Command::new("sh");
            shell.arg("-c");
            shell
        };
        let status = shell
            .arg(command)
            .current_dir(&path)
            .status()
            .context("Could not execute the command inside the worktree")?;
        if !status.success() {
            bail!("Worktree command failed ({status})");
        }
    }
    Ok(())
}

pub fn list(long: bool, json_output: bool) -> Result<()> {
    #[derive(Serialize)]
    struct Entry {
        #[serde(flatten)]
        worktree: Worktree,
        metadata: Option<IntentMetadata>,
    }

    let entries: Vec<Entry> = worktrees()?
        .into_iter()
        .map(|worktree| {
            let metadata = if worktree.bare || worktree.prunable || !(long || json_output) {
                None
            } else {
                IntentMetadata::load(&worktree.path)?
            };
            Ok(Entry { worktree, metadata })
        })
        .collect::<Result<_>>()?;

    if json_output {
        println!("{}", serde_json::to_string_pretty(&entries)?);
        return Ok(());
    }

    for entry in entries {
        let tree = entry.worktree;
        let label =
            tree.branch
                .as_deref()
                .unwrap_or(if tree.bare { "(bare)" } else { "(detached)" });
        println!("{label}  {}", tree.path.display());
        if let Some(metadata) = entry.metadata {
            println!(
                "  Agent: {} | Intent: {} | Scope: {:?} | Risk: {:?}",
                metadata.agent, metadata.intent, metadata.scope, metadata.risk
            );
        }
    }
    Ok(())
}

pub fn clean(all: bool) -> Result<()> {
    let trees = worktrees()?;
    if all {
        let here = current(&trees)?;
        let base = trees
            .first()
            .and_then(|tree| tree.branch.as_deref())
            .context("The primary worktree must be on a branch for clean --all")?;
        let base_ref = branch_ref(base)?;
        let merged = git(
            &here.path,
            &[
                "for-each-ref",
                "--format=%(refname)",
                "--merged",
                &base_ref,
                "refs/heads",
            ],
        )?;
        for tree in trees.iter().skip(1) {
            if tree.path == here.path || tree.locked || tree.prunable {
                continue;
            }
            let Some(branch) = &tree.branch else { continue };
            if !merged
                .lines()
                .any(|candidate| candidate.strip_prefix("refs/heads/") == Some(branch.as_str()))
                || IntentMetadata::load(&tree.path)?.is_none()
                || !git(
                    &tree.path,
                    &[
                        "status",
                        "--porcelain",
                        "--untracked-files=all",
                        "--ignored",
                    ],
                )?
                .is_empty()
            {
                continue;
            }
            git(
                &here.path,
                &["worktree", "remove", "--", path_arg(&tree.path)?],
            )?;
            println!("Removed worktree: {}", tree.path.display());
        }
    }
    git(Path::new("."), &["worktree", "prune"])?;
    println!("Pruned stale Git worktree records.");
    Ok(())
}
