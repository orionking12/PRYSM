# CI incident — PRYSM / 2026-11-11

## Observable facts
- Pull request: https://github.com/orionking12/PRYSM/pull/1
- Workflow: `Rust CI`
- Run: https://github.com/orionking12/PRYSM/actions/runs/43025074742
- Commit: `97319bd34d52ba167d1201f4a891fc00d2fa7f78`
- Conclusion: **failure**.
- Both `windows-latest` and `ubuntu-latest` jobs were marked failed within roughly three seconds of starting.
- The jobs endpoint reported `steps: []` for both jobs.
- Attempts to retrieve downloadable logs returned a `BlobNotFound` response from the log storage backend.

## Diagnostic assessment
There is **no evidence yet** that `cargo check` or `cargo test` ran. A pre-step failure may be a runner provisioning or repository/account Actions configuration problem, but the available records do **not** prove the cause. Do not label the source code as failing until the commands actually run.

## Next checks (human-accessible GitHub UI)
1. Open the workflow run URL above and inspect the error banner in Actions.
2. Verify Actions are enabled for this repository and check organizational/repository workflow action allowlists.
3. Check Actions minute quota, spending limits and runner availability.
4. Verify any required approvals for workflows on first-time pull requests.
5. When the Actions service is available, re-run **failed jobs** and inspect step logs.
6. If jobs begin running, collect `cargo check --locked --all-targets` and `cargo test --locked --all-targets` results and attach them to the release notes.


## Repeat diagnostic (second attempt)
- The failed jobs from run [43043656233](https://github.com/orionking12/PRYSM/actions/runs/43043656233) were re-run through GitHub.
- Attempt 2 also completed with `failure` for both Ubuntu and Windows in around three seconds.
- The jobs reported `runner_id=0`, empty `runner_name`, and `steps=[]`.
- This strengthens the suspicion of a **pre-runner/provisioning/Actions configuration** problem, not a Rust test failure, but the precise cause remains unconfirmed without the GitHub UI message.
- Local fallback scripts were added under `scripts/validate-release.ps1` and `scripts/validate-release.sh`. They have **not** yet been run on the author's machine and do not substitute for CI evidence.

## Root cause confirmed by GitHub UI (2026-11-11)

The repository owner's GitHub Actions screenshot displays the explicit annotation (Spanish): **"El trabajo no se ha iniciado porque su cuenta está bloqueada debido a un problema de facturación."**

This accounts for the missing runners and zero executed job steps. It is a **GitHub account billing restriction**, not a verified Rust compile or test failure. The Ubuntu runner migration warning is informational and not the observed cause.

To continue without paid GitHub Actions, use the proposed Python 3 standard-library script `scripts/prysm_verify.py`, described in `docs/LOCAL_VERIFIER.md`. The verifier's own 5 unit tests passed locally; PRYSM's actual Cargo checks must still run on a host with Rust installed. Do not misreport local evidence as a GitHub CI green result.

## Publication gate
**BLOCKED (GitHub Actions account billing restriction confirmed in UI).** The new workflow is a proposed test configuration, not passed evidence. No merge or release is approved by this document.
