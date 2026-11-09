use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

struct TestDir(PathBuf);

impl TestDir {
    fn create() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("prysm-test-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn checked(command: &mut Command) -> Output {
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "{command:?}\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn git(path: &Path, args: &[&str]) -> String {
    let output = checked(Command::new("git").current_dir(path).args(args));
    String::from_utf8(output.stdout).unwrap()
}

fn prysm(path: &Path, args: &[&str]) -> Output {
    checked(
        Command::new(env!("CARGO_BIN_EXE_prysm"))
            .current_dir(path)
            .args(args),
    )
}

fn prysm_unchecked(path: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_prysm"))
        .current_dir(path)
        .args(args)
        .output()
        .unwrap()
}

fn init_repo(main: &Path) {
    fs::create_dir(main).unwrap();
    git(main, &["init", "-b", "main"]);
    git(main, &["config", "user.name", "PRYSM Test"]);
    git(main, &["config", "user.email", "prysm-test@example.invalid"]);
    git(main, &["config", "commit.gpgSign", "false"]);
    git(main, &["config", "core.autocrlf", "false"]);
}

#[test]
fn same_line_conflict_reports_files_and_keeps_main() {
    let temp = TestDir::create();
    let main = temp.0.join("demo repo");
    init_repo(&main);
    fs::write(main.join("shared.txt"), "endpoint: /v1/users\n").unwrap();
    git(&main, &["add", "--", "shared.txt"]);
    git(&main, &["commit", "-m", "Seed"]);

    for (branch, agent, intent) in [
        ("auth", "Claude", "Add login"),
        ("api", "Codex", "Add pagination"),
    ] {
        prysm(
            &main,
            &[
                "switch", "-c", branch, "--agent", agent, "--intent", intent, "--scope", "api",
            ],
        );
    }

    let root = temp.0.join(".prysm_worktrees").join("demo repo");
    let auth = root.join("auth");
    let api = root.join("api");
    for (tree, content, message) in [
        (&auth, "endpoint: /v1/users?auth=oauth2\n", "Claude edits endpoint"),
        (&api, "endpoint: /v1/users?page=1\n", "Codex edits endpoint"),
    ] {
        fs::write(tree.join("shared.txt"), content).unwrap();
        git(tree, &["commit", "-am", message]);
    }

    prysm(&auth, &["merge", "main"]);
    let kept = git(&main, &["rev-parse", "HEAD"]);
    let kept_short = git(&main, &["rev-parse", "--short", "HEAD"]);

    let failed = prysm_unchecked(&api, &["merge", "main"]);
    let stderr = String::from_utf8_lossy(&failed.stderr);
    println!("{stderr}");
    assert!(!failed.status.success(), "merge should fail on conflict");
    let toplevel = git(&main, &["rev-parse", "--show-toplevel"]);
    assert!(stderr.contains("Merge conflict: api → main"), "{stderr}");
    assert!(
        stderr.contains(&format!("Resolve in worktree: {}", toplevel.trim())),
        "{stderr}"
    );
    assert!(stderr.contains("Conflicted files:\n  shared.txt"), "{stderr}");
    assert!(
        stderr.contains(&format!("main still points to {}", kept_short.trim())),
        "{stderr}"
    );
    assert_eq!(kept, git(&main, &["rev-parse", "HEAD"]));
    assert!(git(&main, &["status", "--porcelain"]).contains("UU shared.txt"));
}

#[test]
fn shared_scope_warns_and_both_branches_land_on_main() {
    let temp = TestDir::create();
    let main = temp.0.join("demo repo");
    fs::create_dir(&main).unwrap();
    git(&main, &["init", "-b", "main"]);
    git(&main, &["config", "user.name", "PRYSM Test"]);
    git(
        &main,
        &["config", "user.email", "prysm-test@example.invalid"],
    );
    git(&main, &["config", "commit.gpgSign", "false"]);
    git(&main, &["commit", "--allow-empty", "-m", "Seed"]);

    let before = git(&main, &["rev-parse", "HEAD"]);
    prysm(
        &main,
        &[
            "switch",
            "-c",
            "auth",
            "--agent",
            "Claude",
            "--intent",
            "Add login",
            "--scope",
            "api",
        ],
    );
    prysm(
        &main,
        &[
            "switch",
            "-c",
            "api",
            "--agent",
            "Codex",
            "--intent",
            "Add pagination",
            "--scope",
            "api",
        ],
    );

    let root = temp.0.join(".prysm_worktrees").join("demo repo");
    let auth = root.join("auth");
    let api = root.join("api");
    for (tree, file, content, message) in [
        (&auth, "login.txt", "OAuth2 login\n", "Add login"),
        (&api, "pagination.txt", "API pagination\n", "Add pagination"),
    ] {
        fs::write(tree.join(file), content).unwrap();
        git(tree, &["add", "--", file]);
        git(tree, &["commit", "-m", message]);
        println!(
            "{message}: {}",
            git(tree, &["rev-parse", "--short", "HEAD"]).trim()
        );
    }

    let dry_run = prysm(&auth, &["merge", "main", "--dry-run"]);
    let warning = String::from_utf8(dry_run.stderr).unwrap();
    assert!(warning.contains("Warning: shared scope with api (Codex): [\"api\"]"));
    assert_eq!(before, git(&main, &["rev-parse", "HEAD"]));
    assert!(git(&main, &["status", "--porcelain"]).is_empty());
    assert!(!main.join("login.txt").exists());
    assert!(!main.join("pagination.txt").exists());
    println!(
        "{}{}main unchanged after dry-run",
        String::from_utf8_lossy(&dry_run.stdout),
        warning
    );

    let first_merge = prysm(&auth, &["merge", "main"]);
    assert!(String::from_utf8_lossy(&first_merge.stderr).contains(&warning));
    assert_ne!(before, git(&main, &["rev-parse", "HEAD"]));
    assert_eq!(
        git(&main, &["log", "-1", "--format=%s"]).trim(),
        "Merge auth into main"
    );
    assert_eq!(git(&main, &["show", "HEAD:login.txt"]), "OAuth2 login\n");

    prysm(&api, &["merge", "main"]);
    assert_eq!(git(&main, &["show", "HEAD:login.txt"]), "OAuth2 login\n");
    assert_eq!(
        git(&main, &["show", "HEAD:pagination.txt"]),
        "API pagination\n"
    );
    assert_eq!(git(&main, &["rev-list", "--count", "main"]).trim(), "3");
    assert!(git(&main, &["status", "--porcelain"]).is_empty());
    println!("{}", git(&main, &["log", "--oneline", "-3", "main"]));
}
