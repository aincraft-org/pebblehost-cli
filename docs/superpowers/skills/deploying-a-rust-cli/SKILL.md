---
name: deploying-a-rust-cli
description: Use when cutting a new cross-platform release of a Rust CLI through GitHub Actions and need versioned artifacts, smoke tests, and a single GitHub Release.
---

# Deploying a Rust CLI

## Overview

Releases are produced by the repository's GitHub Actions workflow, not by hand. Let the workflow compute the version from UTC date and run number, build the matrix, and create one release. After it finishes, verify the release has the expected number of assets and smoke-test at least one binary.

## When to Use

- A Rust CLI has a `.github/workflows/release.yml`.
- The release needs builds for multiple OS/architectures.
- Version format is date-based plus a CI run number.
- You are asked to trigger, monitor, or verify a release.

## Quick Reference

| Step | Action | Notes |
|------|--------|-------|
| Pre-flight | `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test --all-features` | Do not trigger a release with failing lint/tests. |
| Trigger | GitHub UI → Actions → `release.yml` → Run workflow | Use `workflow_dispatch`; do not push a tag manually. |
| Version | Wait for `vYYYY.M.D.run` tag | Cargo version becomes `YYYY.M.D+run` (build metadata). |
| Verify assets | One release with six (or expected count) tarballs/zip | x86_64/aarch64/armv7 Linux, x86_64/aarch64 macOS, x86_64 Windows are typical. |
| Smoke test | Download, extract, run `<binary> --version` | Confirms the artifact matches the tag. |

## Core Pattern

1. **Confirm green.** Lint, clippy, and tests pass before triggering.
2. **Trigger the workflow.** Use `workflow_dispatch` from the GitHub Actions UI or `gh workflow run release.yml` if you are certain of state.
3. **Watch the matrix.** Each target should build and upload.
4. **Check the tag and release.** There should be exactly one release for the new tag with the expected asset count.
5. **Smoke-test.** Download one native artifact, extract it, and run `--version` or `--help`.

## Common Mistakes

- Building and uploading binaries manually instead of using the workflow.
- Manually pushing a release tag before the workflow creates it.
- Re-running the same-day workflow without checking for tag collisions.
- Skipping lint/test before triggering.
- Declaring the release done without downloading and running an artifact.
