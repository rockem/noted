# Copilot instructions for Noted

Purpose: help Copilot sessions make small, correct changes that match this repository's CLI behavior and test strategy.

## Build, test, and lint commands

- Build: `cargo build`
- Run: `cargo run`
  - Typical local run with explicit store: `NOTED_STORE=/path/to/notes cargo run`
- Check: `cargo check`
- Format: `cargo fmt`
- Lint: `cargo clippy`
- Full tests: `cargo test`
- Single test by name: `cargo test create_daily_note_file`
- Single integration test target: `cargo test --test component`

CI workflows:

- `.github/workflows/test.yml` runs `cargo test` on push and pull requests to `main`.
- `.github/workflows/lint.yml` installs `clippy` and `rustfmt`, then runs pre-commit hooks.
- `.github/workflows/release.yml` cross-compiles release binaries (Linux/macOS targets), uploads release artifacts, and
  triggers Homebrew tap formula updates.

## High-level architecture

- `src/main.rs` owns the full runtime flow:
  - Parse CLI args with `clap` (`-e` for quick capture text or piped input).
  - Resolve store path from `NOTED_STORE` or platform data dir + `noted`.
  - Build daily note path as `daily/YYYY/MM/YYYY-MM-DD.md`.
  - Ensure parent directories exist.
  - Either append timestamped capture entries (`**HH:MM** text`) or open the note in `$EDITOR` (fallback `vim`).
- `src/lib.rs` exports shared modules (`errors`, `version`) used by the binary and tests.
- `src/version.rs` embeds Git-derived version metadata via `git-version`; CLI version output is expected to align with
  this constant.
- `src/errors.rs` contains user-facing error constants (for example daily note creation failures), and component tests
  assert on these exact messages.

### Test architecture is integration-first

- `tests/component/*` runs the real `noted` binary through driver helpers.
- `tests/component/support/app_driver.rs` centralizes process spawning and env setup (`NOTED_STORE`, `EDITOR`), plus
  stdin and PTY execution paths.
- `tests/component/support/store_driver.rs` creates isolated temp stores and computes note paths using the same date
  layout as production code.

## Key conventions

- Prefer component/integration coverage for behavior changes in CLI flow, file creation, and editor/capture
  interactions.
- Preserve the daily note storage contract exactly: `daily/<year>/<month>/<YYYY-MM-DD>.md`.
- In tests, use `AppDriver`/`StoreDriver` instead of ad-hoc process or filesystem setup to keep env and cleanup behavior
  consistent.
- When asserting failures, prefer existing exported error constants (`noted::errors::*`) so tests stay aligned with
  user-visible output.

## Existing assistant guidance to preserve

- `CLAUDE.md` reinforces TDD-style development and the driver-based test pattern; follow that structure when adding
  coverage.
- `README.md` is the source for end-user behavior (default storage location semantics, `$EDITOR` fallback, install/usage
  examples). Keep implementation and tests aligned with it.
