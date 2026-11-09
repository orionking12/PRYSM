use crate::{
    intent::IntentMetadata,
    worktree::{branch_ref, current, git, worktrees},
};
use anyhow::{bail, Context, Result};

pub fn run(target: String, squash: bool, dry_run: bool) -> Result<()> {
    branch_ref(&target)?;
    let trees = worktrees()?;
    let source = current(&trees)?;
    let source_branch = source
        .branch
        .as_deref()
        .context("Merge must start on a branch")?;
    if source_branch == target {
        bail!("Source and target are both on {target}");
    }
    let destination = trees
        .iter()
        .find(|tree| {
            tree.branch.as_deref() == Some(target.as_str()) && !tree.prunable && tree.path.is_dir()
        })
        .with_context(|| format!("Check out {target} in a separate worktree first"))?;

    println!("{source_branch} → {target}");
    let mut overlaps = false;
    if let Some(metadata) = IntentMetadata::load(&source.path)? {
        println!("Agent: {} | Intent: {}", metadata.agent, metadata.intent);
        for tree in &trees {
            if tree.path == source.path || tree.bare || tree.prunable {
                continue;
            }
            let Some(other) = IntentMetadata::load(&tree.path)? else {
                continue;
            };
            let shared: Vec<_> = metadata
                .scope
                .iter()
                .filter(|scope| other.scope.contains(scope))
                .collect();
            if !shared.is_empty() {
                overlaps = true;
                eprintln!(
                    "Warning: shared scope with {} ({}): {:?}",
                    tree.branch.as_deref().unwrap_or("(detached)"),
                    other.agent,
                    shared
                );
            }
        }
        if !overlaps {
            println!("No shared scopes found in available intent metadata.");
        }
    } else {
        println!("Source worktree has no intent metadata. Scope comparison skipped.");
    }
    if dry_run {
        return Ok(());
    }

    for tree in [source, destination] {
        if !git(
            &tree.path,
            &["status", "--porcelain", "--untracked-files=all"],
        )?
        .is_empty()
        {
            bail!("Commit or stash pending changes in {}", tree.path.display());
        }
    }

    let mut args = vec!["merge"];
    if squash {
        args.push("--squash");
    } else {
        args.push("--no-edit");
    }
    args.extend(["--", &source.head]);
    if let Err(error) = git(&destination.path, &args) {
        let conflicts = git(
            &destination.path,
            &["diff", "--name-only", "--diff-filter=U", "-z"],
        )
        .unwrap_or_default();
        let files: Vec<&str> = conflicts.split('\0').filter(|file| !file.is_empty()).collect();
        if files.is_empty() {
            return Err(error).with_context(|| {
                format!(
                    "Merge failed in {}. Inspect Git status there",
                    destination.path.display()
                )
            });
        }
        let kept = git(&destination.path, &["rev-parse", "--short", "HEAD"])
            .map(|head| head.trim().to_owned())
            .unwrap_or_default();
        let mut message = format!(
            "Merge conflict: {source_branch} → {target}\nResolve in worktree: {}\nConflicted files:",
            destination.path.display()
        );
        for file in &files {
            message.push_str("\n  ");
            message.push_str(file);
        }
        message.push_str(&format!(
            "\n{target} still points to {kept}. Resolve the files and commit in that worktree."
        ));
        bail!(message);
    }
    if squash
        && !git(
            &destination.path,
            &["diff", "--cached", "--name-only", "-z"],
        )?
        .is_empty()
    {
        let message = format!("Merge {source_branch} into {target}");
        git(&destination.path, &["commit", "-m", &message]).with_context(|| {
            format!(
                "Squash changes are staged in {}, but the commit failed",
                destination.path.display()
            )
        })?;
    }

    println!("Merged into {target} at {}", destination.path.display());
    Ok(())
}
