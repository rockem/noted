# Noted

A fast, minimal CLI note-taking app with daily notes and Git sync.

## Why Noted?

- **Use your own editor** - Vim, Neovim, VS Code, Emacs. Your keybindings, your plugins.
- **Plain Markdown** - No vendor lock-in. Your notes are just `.md` files.
- **Git sync** - Version history, backup, and sync across machines.
- **Fast** - No Electron, no web views. Starts instantly.
- **Scriptable** - Integrate with your shell, cron jobs, and other tools.

## Installation

```bash
cargo install noted
```

## Quick Start

```bash
# Open today's daily note in your editor
noted

# Quick capture without opening editor
noted -e "Remember to call Alice"

# Create a new note
noted new "Project Ideas"

# Search across all notes
noted search "meeting notes"

# List all notes
noted list
```

## Commands

| Command                  | Description                 |
| ------------------------ | --------------------------- |
| `noted`                  | Open today's daily note     |
| `noted -e <text>`        | Append text to today's note |
| `noted new <title>`      | Create a new note           |
| `noted edit <query>`     | Open a note by name         |
| `noted view <query>`     | View note content           |
| `noted list`             | List all notes              |
| `noted list --tag <tag>` | List notes with tag         |
| `noted search <query>`   | Full-text search            |
| `noted delete <query>`   | Delete a note               |
| `noted yesterday`        | Open yesterday's note       |
| `noted daily <date>`     | Open note for specific date |
| `noted sync`             | Sync with Git remote        |

## Note Format

Notes use Markdown with optional YAML frontmatter:

```markdown
---
title: Project Ideas
created: 2026-02-03T10:30:00
tags: [work, ideas]
---

# Project Ideas

Your content here...
```

## Shell Integration

```bash
# Quick capture alias
alias n="noted -e"

# Pipe content into notes
git log --oneline -10 | noted -e

# Use with fzf
noted search "project" | fzf | xargs noted edit
```

## Configuration

`~/.config/noted/config.toml`:

```toml
[general]
editor = "nvim"
notes_dir = "~/.noted"

[daily]
template = "templates/daily.md"
```

## License

MIT
