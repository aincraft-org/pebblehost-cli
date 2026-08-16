---
name: extending-a-rust-cli
description: Use when adding a new typed subcommand to a Rust CLI that is backed by an OpenAPI-style or bundled endpoint inventory and uses clap, reqwest, and wiremock tests.
---

# Extending a Rust CLI

## Overview

Map a new command from the API inventory first, then add the `clap` representation, wire the request, and add a `wiremock` test. Finish with `cargo fmt`, `clippy`, and `cargo test`.

## When to Use

- The CLI has an `operations` subcommand or bundled `operations.json`.
- You need a new `clap` subcommand for an existing API endpoint.
- Tests use `wiremock` to mock the HTTP server.

## Quick Reference

| Step | Where | What to do |
|------|-------|------------|
| Find operation | `src/operations.json` or `<cli> operations` | Confirm method, path, parameters, body. |
| Add `Command` | `src/main.rs` | Add a new `Subcommand` variant or nested subcommand. |
| Add args | Next to the variant | `#[derive(Args)]` struct with `#[arg(...)]`. |
| Wire request | `execute()` dispatch or `Api` | Build path, method, query, body; call `Api::request`. |
| Test | `tests/` or bottom of `src/main.rs` | `wiremock` asserting method, path, auth header, status, body. |
| Verify | Terminal | `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`. |

## Core Pattern

1. **Read the inventory.** Find the operation by ID or path. Note the exact HTTP method, path template, and whether the body is empty or JSON.
2. **Design the CLI surface.** Prefer a typed subcommand; only add an `api-call` example if no typed command exists.
3. **Preserve compatibility.** Do not rename or remove existing variants unless the user explicitly asks. If a flat arg already exists (e.g. `backups <server>`), nest it with `#[derive(Subcommand)]` and add a `List` variant for the old behavior.
4. **Implement the request.** Use the central `Api` helper and the existing request pattern (Bearer token, `Accept: application/json`, etc.).
5. **Add a test.** Use `wiremock` to assert the method, path, headers, and response handling.
6. **Lint and test.** Run `cargo fmt --check`, `cargo clippy`, and `cargo test` before finishing.

## Common Mistakes

- Guessing the path/method instead of checking `operations.json`.
- Adding a command without a corresponding `wiremock` test.
- Using `#[derive(Parser)]` on a nested command instead of `#[derive(Subcommand)]`.
- Forgetting to update the `execute` match arm.
- Converting a flat arg like `backups <server>` to a nested subcommand without adding a `List` variant for the old behavior.
- Skipping `cargo clippy` or `cargo fmt --check`.
