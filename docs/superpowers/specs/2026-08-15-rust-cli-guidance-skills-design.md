# Rust CLI Guidance Skills Pack

## Goal

Create three small, reusable agentic skills for working with Rust CLIs. Each skill uses `pebblehost-cli` as the concrete example, but the guidance is intentionally general so it applies to any Rust CLI with similar patterns.

## Skills

### 1. `using-a-rust-cli`

**Trigger:** An agent needs to run a Rust CLI, especially one that wraps an HTTP API.

**Core guidance:**
- Locate and read the CLI’s README / `--help` first.
- Prefer environment variables (`PEBBLEHOST_API_TOKEN`) over `--token` in scripts, but use `--token` for overrides.
- Understand `--base-url` and when to override it.
- Use `--json` for machine-readable output.
- Use the generic escape hatch (`api-call`, `operations`, etc.) when a typed command does not exist.
- Common mistakes: missing token, wrong base URL, trailing slashes, passing `path` without leading `/`.

### 2. `deploying-a-rust-cli`

**Trigger:** An agent needs to build, package, or release a Rust CLI.

**Core guidance:**
- Use GitHub Actions with a build matrix for cross-compilation.
- Prefer `workflow_dispatch` for releases triggered by a human rather than on every push.
- Versioning: pick a scheme and keep release tags, artifact names, and Cargo versions aligned.
- Package per target (`x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`, etc.).
- Create GitHub Releases from CI and attach artifacts.
- Smoke test downloaded artifacts before announcing a release.

### 3. `extending-a-rust-cli`

**Trigger:** An agent needs to add a new subcommand to an existing clap-based Rust CLI.

**Core guidance:**
- Find the API endpoint (OpenAPI spec, `operations` inventory, or `api-call` output).
- Add a `clap::Args` struct and a `Command` enum variant.
- Map path params, query params, and JSON body fields.
- Use the existing `Api::request` helper and match response handling.
- Add a wiremock test that asserts the exact path, method, headers, query, and body.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` before finishing.

## Location

Skills will live under the runtime skills directory (`~/.claude/skills/` and `~/.agents/skills/`) and can be committed to a personal fork if the user wants to share them.

## Testing Plan

Per `writing-skills`, each skill will be tested by running a pressure scenario with a subagent that does not have the skill, then with the skill, and iterating until the agent follows the guidance. Scenarios:
- Using: ask an agent to “check account info with the pebblehost CLI.”
- Deploying: ask an agent to “cut a new cross-platform release.”
- Extending: ask an agent to “add a backups create command to the CLI.”

## Acceptance Criteria

- Three `SKILL.md` files with proper YAML frontmatter.
- Each skill under 500 words.
- Each skill passes its pressure scenario with a subagent.
- Spec is committed to the repo.
