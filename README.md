# Noted

Noted is a command-line note-taking application built in Rust.
It provides a simple, distraction-free way to capture notes and maintain a daily journal,
with all data stored as Markdown files.

Today, `noted` opens or creates your daily note in your preferred editor.

## Installation

Homebrew:

```sh
brew install rockem/homebrew-tap/noted
```

## Usage

```sh
# Open today's note in your default editor
noted

# Show the current version
noted --version
```

Notes are stored in `{data_dir}/noted` by default (e.g. `~/Library/Application Support/noted` on macOS).
Override with the `NOTED_STORE` environment variable:

```sh
NOTED_STORE=~/notes noted
```

Editor selection uses `$EDITOR` when set, and falls back to `vim`.

## Roadmap (Planned)

- Git sync for pull/push workflows
- Note list and search
- Tags and organization
- Templates
- Plugin system
