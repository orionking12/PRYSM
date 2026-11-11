"""Standard-library tests for PRYSM offline verifier logic.

These tests check the verifier, not the Rust project itself.
"""
from pathlib import Path
import json
import tempfile
import unittest
from unittest.mock import patch

import prysm_verify


class VerifierTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="verify-test-")
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.repo = self.root / "project"
        self.repo.mkdir()
        (self.repo / "Cargo.toml").write_text('[package]\nname="prysm"\nversion="0.1.1"\n')
        (self.repo / "Cargo.lock").write_text('# lockfile\n')
        self.out = self.root / "evidence"

    def _meta(self, args, cwd):
        if "--porcelain" in args:
            return ""
        if "rev-parse" in args:
            return "a" * 40
        return "fake-tool-version"

    def _command(self, args, cwd, output, timeout):
        output.write_text("fake tool executed\n")
        return {"command": args, "status": "PASS", "exit_code": 0,
                "started_utc": "2026-11-11T00:00:00+00:00",
                "finished_utc": "2026-11-11T00:00:01+00:00",
                "log": output.name, "log_sha256": prysm_verify.sha256(output)}

    def test_pass_contains_sha_provenance_and_two_checks(self):
        with patch.object(prysm_verify.shutil, "which", side_effect=lambda n: n), \
             patch.object(prysm_verify, "short_capture", side_effect=self._meta), \
             patch.object(prysm_verify, "run_command", side_effect=self._command):
            code = prysm_verify.verify(self.repo, self.out, 8, False, False)
        report = json.loads((self.out / "manifest.json").read_text())
        self.assertEqual(code, 0)
        self.assertEqual(report["result"], "PASS")
        self.assertEqual(len(report["checks"]), 2)
        self.assertEqual(len(report["source_sha256"]["Cargo.toml"]), 64)
        self.assertEqual(report["commit"], "a" * 40)

    def test_failure_is_not_claimed_pass(self):
        def fail(args, cwd, output, timeout):
            v = self._command(args, cwd, output, timeout)
            v["status"], v["exit_code"] = "FAIL", 101
            return v
        with patch.object(prysm_verify.shutil, "which", side_effect=lambda n: n), \
             patch.object(prysm_verify, "short_capture", side_effect=self._meta), \
             patch.object(prysm_verify, "run_command", side_effect=fail):
            code = prysm_verify.verify(self.repo, self.out, 8, False, False)
        report = json.loads((self.out / "manifest.json").read_text())
        self.assertEqual(code, 1)
        self.assertEqual(report["result"], "FAIL")
        self.assertEqual(len(report["checks"]), 1)

    def test_missing_cargo_blocks_and_still_writes_report(self):
        with patch.object(prysm_verify.shutil, "which", side_effect=lambda n: None if n == "cargo" else "git"):
            code = prysm_verify.verify(self.repo, self.out, 8, False, False)
        report = json.loads((self.out / "manifest.json").read_text())
        self.assertEqual(code, 2)
        self.assertEqual(report["result"], "BLOCKED")
        self.assertIn("cargo", report["reason"])

    def test_missing_project_blocks(self):
        (self.repo / "Cargo.toml").unlink()
        code = prysm_verify.verify(self.repo, self.out, 8, False, False)
        self.assertEqual(code, 2)
        self.assertEqual(json.loads((self.out / "manifest.json").read_text())["result"], "BLOCKED")

    def test_command_failure_captured_in_log(self):
        from sys import executable
        res = prysm_verify.run_command(
            [executable, "-c", 'import sys; print("intentional failure");sys.exit(7)'],
            self.repo, self.root / "failed.log", 5)
        self.assertEqual(res["status"], "FAIL")
        self.assertEqual(res["exit_code"], 7)
        self.assertIn("intentional failure", (self.root / "failed.log").read_text())


if __name__ == "__main__":
    unittest.main()
