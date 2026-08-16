---
name: using-a-rust-cli
description: Use when asked to run, inspect, script, or troubleshoot a Rust CLI that wraps a remote API with clap, reqwest, and an authentication token.
---

# Using a Rust CLI

## Overview

A Rust CLI with `clap` and `reqwest` is usually the source of truth for itself. Discover commands through `--help` and the bundled API inventory, keep secrets in the environment, and use the generic `api-call` escape hatch only when no typed command exists.

## When to Use

- Running or scripting a Rust CLI against an API.
- You are not sure which subcommand maps to an endpoint.
- The CLI has a bundled `operations` or OpenAPI inventory.
- Output needs to be machine-readable (JSON) or needs a non-production base URL.

## Quick Reference

| Goal | Typical command | Notes |
|------|-----------------|-------|
| Build + run from repo | `cargo run -- <args>` | `cargo run -- --help` is the fastest first step. |
| List all typed commands | `<cli> --help` | Read global flags carefully. |
| List API endpoints | `<cli> operations` | Filter with `--method GET/POST/...`. |
| Authenticate safely | `export API_TOKEN=...` | Prefer env; avoid `--token` in scripts or logs. |
| Change base URL | `BASE_URL=... <cli> ...` or `--base-url` | Use for test/staging panels. |
| JSON output | `<cli> --json <subcommand>` | Stable for scripts. |
| Generic request | `<cli> api-call POST /api/client/servers/123/backups` | Use only when no typed command exists. |

## Core Pattern

1. **Locate the binary.** Use the installed binary if present; otherwise `cargo run --` from the repo root.
2. **Read help.** Global flags (`--token`, `--base-url`, `--json`) apply to every subcommand.
3. **Set auth.** Use an environment variable so the token does not appear in shell history or logs.
4. **Discover.** If the exact command is unclear, run `operations` and map the endpoint to a typed command or use `api-call`.
5. **Verify first.** In baselines or tests, mock the API or target a trusted panel; do not hit production with a real token.

## Common Mistakes

- Running `./target/debug/pebblehost` without checking which binary is current; use `cargo run --`.
- Passing secrets as `--token` in shared scripts or screenshots.
- Skipping `--help` and guessing argument names.
- Using `api-call` when a typed subcommand already exists.
- Hitting the live API during a baseline or test run.
