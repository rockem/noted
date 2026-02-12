# Noted

A fast, minimal CLI note-taking app with daily notes and Git sync.

## Overview

Noted is a command-line note-taking application built in Rust.
It provides a simple, distraction-free way to capture notes and maintain a daily journal,
with all data stored as Markdown files and synced via Git.

## Problem Statement

Existing note-taking apps are often:

- Bloated with features and slow to open
- Locked into proprietary formats or cloud services
- Not keyboard-friendly for developers and terminal users
- Missing simple daily note workflows

Noted solves this by providing a fast CLI tool that stores plain Markdown files
in a Git repository, giving users full control over their data.

## Target Users

- Developers and terminal power users
- People who prefer keyboard-driven workflows
- Users who want ownership of their notes (no vendor lock-in)
- Anyone who maintains a daily log or journal

## Core Features

### 1. Note Management

- Create, view, edit, and delete notes
- List all notes with filtering options
- Open notes in your preferred editor (`$EDITOR`)

### 2. Daily Notes (core plugin)

- Quick command to create/edit today's note
- Quick add content to the daily note from command line
- Access yesterday's note or any specific date
- Automatic date-based file organization

### 3. Tags & Organization

- Add tags to notes via front matter or inline `#tags`
- Filter and list notes by tags
- Folder-based organization support

### 4. Search

- Full-text search across all notes
- Search by title, content, or tags
- Fast fuzzy matching

### 5. GitHub Sync (core plugin)

- Store notes in a Git repository
- Automatic sync
- Work offline, sync when ready

## Technical Requirements

| Requirement | Specification          |
| ----------- | ---------------------- |
| Languages   | Rust, Lua              |
| Note Format | Markdown (.md)         |
| Storage     | Local filesystem + Git |
| Sync        | GitHub (via git)       |
| Config      | TOML file              |
| Platforms   | macOS, Linux, Windows  |

## CLI Commands (core)

```text
noted                     # Open today's daily note
noted -e <new entry>      # Add a new entry to today's daily note
noted new <title>         # Create a new note
noted edit <query>        # Edit a note (alias for open)
noted list                # List all notes
noted list --tag <tag>    # List notes with specific tag
noted search <query>      # Full-text search
noted delete <query>      # Delete a note
noted today               # Open today's daily note
noted yesterday           # Open yesterday's daily note
noted daily <date>        # Open daily note for specific date (YYYY-MM-DD)
noted sync                # Pull then push changes to GitHub
noted config              # Open config file
noted init                # Initialize noted in current directory
```

## Data Structure

```text
~/.noted/                 # Default notes directory (configurable)
├── .git/                 # Git repository
├── daily/                # Daily notes
│   └── 2026/
│       └── 01/
│           ├── 20.md     # Daily note for 2026-01-20
│           └── 21.md
├── notes/                # Regular notes
|   └── ideas
|       └── shadow-env.md
│   ├── project-ideas.md
│   └── meeting-notes.md
└── templates/            # Note templates (optional)
    └── daily.md
```

## Note Format

Notes are stored as Markdown with optional YAML frontmatter:

```markdown
---
title: Meeting Notes
tags: [work, meetings]
created: 2026-01-20T10:30:00
updated: 2026-01-22T19:02:00
---

# Meeting Notes

Your content here...
```

### Frontmatter Handling

Noted trusts the user with their content. If frontmatter is missing or modified:

**Behavior:**

- Noted will warn if frontmatter is missing or malformed
- Noted will NOT automatically modify user content
- File system metadata (mtime, ctime) is used as fallback for dates

**Fix command:**

```bash
noted doctor                      # Checks for problems with files
noted doctor fix my-note          # Regenerate frontmatter for a note
noted doctor fix --all            # Fix all notes missing frontmatter
```

The `fix` command will:

- Add missing frontmatter fields
- Use file creation time for `created` if missing
- Use file modification time for `updated` if missing
- Extract title from first `# heading` if missing
- Extract inline `#tags` to frontmatter tags array

## Configuration

`.config/noted/config.toml`:

```toml
[general]
editor = "vim"                    # Default: $EDITOR or "vim"
notes_dir = "~/.noted"            # Where notes are stored
auto_save = true

[daily]
template = "templates/daily.md"   # Optional template for daily notes
format = "%Y/%m/%d"               # Date format for daily note paths
```

## GitHub Sync Workflow

1. **Initialize**: `noted init` creates the notes directory and initializes a Git repo
2. **Work**: Notes are auto-committed on save (if enabled)
3. **Sync**: `noted sync` pulls remote changes, rebases local, and pushes

## Future Ideas

- **Templates**: Custom templates for different note types
- **Encryption**: Optional GPG encryption for sensitive notes
- **Export**: Export notes to PDF, HTML, or other formats
- **Hooks**: Pre/post save hooks for custom workflows
- **TUI mode**: Optional interactive terminal UI
- **Backlinks**: Wiki-style `[[note]]` linking between notes
- **Tasks**: Allow to view tasks from notes
- **Quote of the day**: Insert quote of the day to daily notes

### LLM-Powered Features

#### **Search & Retrieval**

- **Semantic search**: Find notes by meaning, not just keywords ("notes about that project deadline")
- **Question answering**: Ask questions across all notes ("When did I last meet with Sarah?")

#### **Writing Assistance**

- **Smart summarization**: Generate weekly/monthly summaries from daily notes
- **Auto-tagging**: Suggest tags or categories based on note content
- **Continuation prompts**: Suggest what to write based on context and patterns

#### **Organization**

- **Topic extraction**: Automatically identify themes across notes over time
- **Linking suggestions**: Surface related notes ("This note relates to your entry from March 3rd")
- **Timeline generation**: Build narrative timelines for projects mentioned across notes

#### **Reflection & Insights**

- **Mood/sentiment tracking**: Analyze emotional patterns over time
- **Goal tracking**: Extract and track progress on goals mentioned in notes
- **Weekly digest**: Generate summaries of accomplishments based on notes

#### **Practical Utilities**

- **Action item extraction**: Pull out TODOs and commitments from free-form text
- **Meeting prep**: Summarize everything noted about a topic before meetings
- **Journal prompts**: Generate personalized prompts based on writing patterns

### Plugin System

Noted supports a Lua-based plugin system for extensibility.
Plugins are self-contained and require no external dependencies.

#### Plugin Structure

```text
~/.config/noted/plugins/
├── git-sync/
│   ├── plugin.toml       # Metadata + declarations
│   └── init.lua          # Entry point
├── llm-search/
│   ├── plugin.toml
│   ├── init.lua
│   └── lib/              # Additional modules
│       └── embeddings.lua
└── 1on1/
    ├── plugin.toml
    └── init.lua
```

#### Plugin Manifest (`plugin.toml`)

```toml
[plugin]
name = "llm-search"
version = "0.1.0"
description = "Semantic search using LLMs"

# Extend existing functionality
[providers]
search = "search_handler"

# Add new commands
[commands]
ask = { handler = "ask_handler", description = "Ask questions about your notes" }

# React to events
[hooks]
after_save = "on_save"
```

#### Plugin Types

| Type         | Purpose                | Example                   |
| ------------ | ---------------------- | ------------------------- |
| **Provider** | Extend core capability | LLM search, S3 sync       |
| **Command**  | Add new subcommand     | `noted ask`, `noted 1on1` |
| **Hook**     | React to events        | Auto-tag on save          |

#### Noted API for Plugins

```lua
noted.get_all_notes()        -- Returns all notes with metadata
noted.get_note(path)         -- Get single note content
noted.create_note(path, content)
noted.config                 -- Access user config
noted.store_path             -- Notes directory
noted.print(text)            -- Output to user
noted.http.get(url)          -- HTTP requests
noted.http.post(url, body)
```

#### Plugin Management

```bash
noted plugin install https://github.com/user/llm-search  # From git
noted plugin install ./my-plugin                          # From local path
noted plugin list                                         # List installed
noted plugin remove llm-search                            # Remove
```

#### Plugin Development

```bash
noted plugin new my-plugin        # Scaffold new plugin
noted plugin dev my-plugin        # Hot reload during development
noted plugin test my-plugin       # Run plugin tests
noted plugin validate my-plugin   # Validate before publishing
noted plugin repl my-plugin       # Interactive REPL with noted API
```

### Sharing Features

Methods to share notes without requiring admin permissions.

#### Clipboard

```bash
noted copy note.md          # Copy note content to clipboard
noted copy today            # Copy today's daily note
```

#### Gist

```bash
noted share note.md --gist              # Create secret GitHub gist
noted share note.md --gist --public     # Create public gist
```

#### Email (for Slack email-to-channel)

```bash
noted share note.md --email channel@workspace.slack.com
```

### Example Plugins

#### 1:1 Meeting Manager

A plugin for managing one-on-one meetings with direct reports.

```bash
noted 1on1 alice                    # Open/create today's 1:1 with Alice
noted 1on1 alice --prep             # Show index + last 3 meetings + open items
noted 1on1 list                     # List all direct reports
noted 1on1 alice --action "..."     # Add to running topics
```

Creates an index note per person with links to individual meeting notes:

```markdown
# Alice Chen

Role: Senior Engineer
Started: 2024-03

## Quick Reference

- Career goal: Staff Engineer
- Current focus: Platform reliability

## Recent 1:1s

- [[2026-02-02]] - Project X, learning goals
- [[2026-01-26]] - Q1 review

## Running Topics

- [ ] Follow up on mentorship pairing
- [ ] Discuss promotion timeline
```

#### Git Sync

Automatic Git synchronization before/after noted commands.

```toml
[plugin]
name = "git-sync"

[hooks]
before_command = "pull_changes"
after_command = "push_changes"
```

#### LLM Search Provider

Extends the built-in search with semantic capabilities.

```toml
[plugin]
name = "llm-search"

[providers]
search = "semantic_search"

[config]
model = "claude-3-haiku"
api_key_env = "ANTHROPIC_API_KEY"
```

```bash
# Unified search - uses both built-in and LLM providers
noted search "meetings about project deadlines"
```

## Development Phases

### Phase 1: Core MVP

- Basic note CRUD operations
- Daily notes functionality
- Local file storage
- List filtering

### Phase 2: Sync plugin

- Basic support for internal plugins
- First plugin: sync with git on start/end
- New sync command for manual sync

### Phase 3: Template & Sharing

- Allow to work with templates (as a plugin?)
- Allow to share a note via gist (as a plugin)

## License

MIT
