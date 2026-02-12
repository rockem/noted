# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Noted is a fast, minimal CLI note-taking app built in Rust.
It stores notes as Markdown files with Git sync. Currently in early MVP stage.

## Build & Development Commands

```bash
# Build
cargo build

# Run
NOTED_STORE=/path/to/notes cargo run

# Run tests
cargo test

# Run a single test
cargo test create_daily_note_file

# Check without building
cargo check

# Format code
cargo fmt

# Lint
cargo clippy
```

## Environment Variables

- `NOTED_STORE` - Required. Path to the notes directory.

## Architecture

### Test Infrastructure

Tests use a driver pattern in `tests/support/`:

- **AppDriver** (`app_driver.rs`) - Wraps the `noted` binary execution. Sets `NOTED_STORE` env var and runs the CLI.
- **StoreDriver** (`store_driver.rs`) - Manages a temporary notes directory. Creates temp dir,
  provides assertions for file existence.

Integration tests run the actual binary via `assert_cmd` crate.

Example test pattern:

```rust
let store = StoreDriver::new();        // Creates temp directory
let app = AppDriver::new(store.path());
app.run();                              // Runs `noted` binary
store.today_note_file_created();        // Assert file exists
```

### Planned Architecture (from PRD)

- Lua-based plugin system for extensibility
- Plugin types: Providers (extend core), Commands (add subcommands), Hooks (react to events)
- Notes stored with YAML frontmatter (title, created, updated, tags)
- Config via TOML at `~/.config/noted/config.toml`
