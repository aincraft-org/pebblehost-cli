# Rust CLI Guidance Skills Pack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` for inline execution. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create three reusable agentic skills (`using-a-rust-cli`, `deploying-a-rust-cli`, `extending-a-rust-cli`) and verify each with a baseline agent scenario.

**Architecture:** Each skill is a `SKILL.md` in the runtime skills directory (`~/.claude/skills/<name>/`). Skills are technique/reference style, use pebblehost-cli as the running example, and follow the `SKILL.md` structure from `writing-skills`.

**Tech Stack:** Markdown, `clap` patterns for Rust CLI, GitHub Actions, `wiremock`, `reqwest`.

## Global Constraints

- Each skill under 500 words.
- Each skill gets a baseline scenario run first, then the skill is written, then the same scenario is re-run to verify.
- Skills live under `~/.claude/skills/` and `~/.agents/skills/`.
- `pebblehost-cli` is the concrete example; skill names and triggers are general.
- README / repo is not modified by this work (only docs already committed).

---

### Task 1: Baseline tests for all three skills

**Files:**
- None yet; subagent will not have the new skills.

**Interfaces:**
- Input: Three short, non-destructive scenarios.
- Output: For each, the agent’s steps and the gaps the skill must fill.

- [ ] **Step 1: Run baseline for `using-a-rust-cli`**
  Prompt an agent to "use the pebblehost CLI to check the current account." It should discover it needs a token, but may miss `--help`, env var, or the generic `api-call`/`operations` fallbacks. Record the gaps.

- [ ] **Step 2: Run baseline for `deploying-a-rust-cli`**
  Prompt an agent to "cut a new cross-platform release of pebblehost-cli." It may skip the build matrix, version scheme, or smoke-testing. Record the gaps.

- [ ] **Step 3: Run baseline for `extending-a-rust-cli`**
  Prompt an agent to "add a `backups create` command to the pebblehost CLI." It may forget tests, `clap` Args, `wiremock`, or `cargo clippy`. Record the gaps.

---

### Task 2: Write `using-a-rust-cli` skill

**Files:**
- Create: `~/.claude/skills/using-a-rust-cli/SKILL.md`

**Interfaces:**
- Produces: a skill file with frontmatter, overview, triggers, common commands, `api-call`/`operations`, and mistakes.

- [ ] **Step 1: Create directory and SKILL.md**
  Use `mkdir -p ~/.claude/skills/using-a-rust-cli` and write the file.

- [ ] **Step 2: Validate skill structure**
  Check frontmatter `name` and `description`, word count, and no narrative.

- [ ] **Step 3: Run with skill**
  Re-run the baseline scenario with the skill loaded; verify the agent checks `--help`, uses the token, and runs the right command.

---

### Task 3: Write `deploying-a-rust-cli` skill

**Files:**
- Create: `~/.claude/skills/deploying-a-rust-cli/SKILL.md`

**Interfaces:**
- Produces: a skill file covering GitHub Actions matrix, `workflow_dispatch`, versioning, artifacts, release creation, smoke testing.

- [ ] **Step 1: Create directory and SKILL.md**
  Write the file.

- [ ] **Step 2: Validate structure**
  Frontmatter, word count, no narrative.

- [ ] **Step 3: Run with skill**
  Re-run baseline scenario and verify the agent proposes a matrix, versioned tag, and smoke test.

---

### Task 4: Write `extending-a-rust-cli` skill

**Files:**
- Create: `~/.claude/skills/extending-a-rust-cli/SKILL.md`

**Interfaces:**
- Produces: a skill file covering OpenAPI lookup, `clap` Args, `Command` enum variant, request mapping, `wiremock` tests, fmt/clippy/test.

- [ ] **Step 1: Create directory and SKILL.md**
  Write the file.

- [ ] **Step 2: Validate structure**
  Frontmatter, word count, no narrative.

- [ ] **Step 3: Run with skill**
  Re-run baseline scenario and verify the agent adds a typed command with a test and runs lint.

---

### Task 5: Final verification and reporting

**Files:**
- Use: the three `~/.claude/skills/<name>/SKILL.md` files

- [ ] **Step 1: Word count check**
  Run `wc -w` on each SKILL.md and confirm ≤ 500.

- [ ] **Step 2: Smoke test each skill**
  Quick `cat` and `grep` for required sections: frontmatter, overview, when-to-use, quick reference, common mistakes.

- [ ] **Step 3: Report paths to user**
  List the three skill paths and key takeaways.

---

## Verification

- [ ] Each skill has a documented baseline failure or gap.
- [ ] Each skill passes its re-run scenario.
- [ ] Each SKILL.md is under 500 words.
- [ ] Each SKILL.md has valid YAML frontmatter with `name` and `description`.
