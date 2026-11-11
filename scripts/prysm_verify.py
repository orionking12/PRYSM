#!/usr/bin/env python3
"""PRYSM local test gate; no GitHub Actions, API tokens, or third-party Python deps.

Exit 0: all required commands executed successfully.
Exit 1: an executed check failed.
Exit 2: prerequisites missing or required check could not execute.

Evidence is self-reported by this computer; hashes are tamper-evident reference
values, not cryptographic signatures or an independent audit.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def run_command(args: list[str], cwd: Path, output: Path, timeout: int) -> dict:
    start = utc_now()
    try:
        done = subprocess.run(
            args,
            cwd=str(cwd),
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            timeout=timeout,
            check=False,
        )
        content = done.stdout or b""
        result = "PASS" if done.returncode == 0 else "FAIL"
        exit_code = done.returncode
    except subprocess.TimeoutExpired as ex:
        content = (ex.stdout or b"") + b"\n[VERIFIER] command timed out\n"
        result = "BLOCKED"
        exit_code = None
    except OSError as ex:
        content = f"[VERIFIER] could not start command: {type(ex).__name__}: {ex}\n".encode()
        result = "BLOCKED"
        exit_code = None
    output.write_bytes(content)
    return {
        "command": args,
        "status": result,
        "exit_code": exit_code,
        "started_utc": start,
        "finished_utc": utc_now(),
        "log": output.name,
        "log_sha256": sha256(output),
    }


def short_capture(args: list[str], cwd: Path) -> str | None:
    try:
        p = subprocess.run(args, cwd=str(cwd), stdin=subprocess.DEVNULL,
                           stdout=subprocess.PIPE, stderr=subprocess.DEVNULL,
                           check=False, timeout=8)
        return p.stdout.decode("utf-8", errors="replace").strip() if p.returncode == 0 else None
    except (OSError, subprocess.TimeoutExpired):
        return None


def verify(repo: Path, out: Path, timeout: int, offline: bool, fmt: bool) -> int:
    repo = repo.resolve()
    out = out.resolve()
    out.mkdir(parents=True, exist_ok=True)
    report = {
        "schema": "prysm-local-verifier/1",
        "project": "PRYSM",
        "created_utc": utc_now(),
        "host": {"os": platform.platform(), "python": platform.python_version()},
        "repository": str(repo),
        "commit": None,
        "dirty_worktree": None,
        "source_sha256": {},
        "checks": [],
        "result": "BLOCKED",
        "limitations": "Local self-reported evidence, NOT an independent signature or security certification.",
    }
    exit_status = 2
    try:
        for name in ("Cargo.toml", "Cargo.lock"):
            path = repo / name
            if path.is_file():
                report["source_sha256"][name] = sha256(path)
        if not (repo / "Cargo.toml").is_file():
            report["reason"] = "Cargo.toml not found in repo directory"
            return exit_status
        git = shutil.which("git")
        cargo = shutil.which("cargo")
        if not git or not cargo:
            report["reason"] = "Required tool missing: " + ", ".join(x for x, v in (("git", git), ("cargo", cargo)) if not v)
            return exit_status
        report["tools"] = {
            "git": short_capture([git, "--version"], repo),
            "cargo": short_capture([cargo, "--version"], repo),
        }
        report["commit"] = short_capture([git, "rev-parse", "HEAD"], repo)
        dirty = short_capture([git, "status", "--porcelain"], repo)
        report["dirty_worktree"] = None if dirty is None else bool(dirty)
        if report["commit"] is None or dirty is None:
            report["reason"] = "Not a readable Git checkout: cannot identify commit and worktree state"
            return exit_status
        cargo_flags = ["--locked"] + (["--offline"] if offline else [])
        commands = [
            ("check", [cargo, "check", *cargo_flags, "--all-targets"]),
            ("test", [cargo, "test", *cargo_flags, "--all-targets"]),
        ]
        if fmt:
            commands.append(("format", [cargo, "fmt", "--all", "--", "--check"]))
        for label, args in commands:
            result = run_command(args, repo, out / f"{label}.log", timeout)
            report["checks"].append(result)
            print(f"  {label}: {result['status']} (log: {out / result['log']})", flush=True)
            if result["status"] != "PASS":
                report["result"] = result["status"]
                report["reason"] = f"Required step {label} did not pass"
                return 1 if result["status"] == "FAIL" else 2
        report["result"] = "PASS"
        report["note"] = ("Source checks passed but worktree is dirty; "
                          "not an immutable release candidate." if report["dirty_worktree"]
                          else "Source checks passed on the recorded Git commit.")
        return 0
    finally:
        report["finished_utc"] = utc_now()
        manifest = out / "manifest.json"
        tmp = out / "manifest.json.tmp"
        tmp.write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
        tmp.replace(manifest)
        print(f"  RESULT: {report['result']} — {manifest}", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description="Local, auditable PRYSM release checks")
    parser.add_argument("--repo", type=Path, default=Path(__file__).resolve().parents[1])
    parser.add_argument("--out", type=Path, default=None,
                        help="Report directory (default: fresh system temp directory)")
    parser.add_argument("--timeout", type=int, default=900, help="Seconds per Cargo command")
    parser.add_argument("--offline", action="store_true", help="Use cached Cargo crates only")
    parser.add_argument("--format", action="store_true", help="Also enforce cargo fmt --check")
    args = parser.parse_args()
    if args.timeout < 1:
        parser.error("--timeout must be greater than zero")
    out = args.out or Path(tempfile.mkdtemp(prefix="prysm-verify-"))
    return verify(args.repo, out, args.timeout, args.offline, args.format)


if __name__ == "__main__":
    sys.exit(main())
