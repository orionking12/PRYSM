# PRYSM CLI usage

| Command | Behavior |
| --- | --- |
| `prysm switch <branch>` | Prints the path of a worktree for the branch, creating the worktree if needed. Pass `-c` or `--create` to create the branch too. |
| `prysm list` | Lists Git worktrees. Pass `--long` for intent metadata or `--json` for structured output. |
| `prysm merge <target>` | From the source worktree, integrates its committed changes into the target branch in a separate worktree. Keep both worktrees clean. |
| `prysm clean` | Prunes stale Git worktree records. `--all` also removes eligible, clean PRYSM worktrees whose branch tips are reachable from the primary worktree's branch. |

`switch` accepts `--agent`, `--intent`, `--scope` (comma-separated labels), and `--risk` (`low`, `med`, or `high`). Use `--exec` to run a shell command in the selected worktree, or change directories to the printed path yourself. New metadata lives in each worktree's Git administrative directory. PRYSM can also read legacy `.prysm/intent.json` metadata.

`merge --dry-run` only compares declared scopes. Scope overlap is advisory. Squash is on by default. Use `--squash=false` to use a regular Git merge and retain the source history. If Git reports a conflict, `merge` exits with an error, lists the conflicted files, prints the target worktree where they must be resolved, and confirms the commit the target still points to.

A squash commit usually leaves the original source tip outside the target's history. Such worktrees are retained by `clean --all`.
