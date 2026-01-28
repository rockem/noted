# Noted

A fast, minimal CLI note-taking app with daily notes and GitHub sync.

## Overview

Noted is a command-line note-taking application built in Rust. It provides a simple, distraction-free way to capture notes and maintain a daily journal, with all data stored as Markdown files and synced via GitHub.

## Problem Statement

Existing note-taking apps are often:

- Bloated with features and slow to open
- Locked into proprietary formats or cloud services
- Not keyboard-friendly for developers and terminal users
- Missing simple daily note workflows

Noted solves this by providing a fast CLI tool that stores plain Markdown files in a Git repository, giving users full control over their data.

## Target Users

- Developers and terminal power users
- People who prefer keyboard-driven workflows
- Users who want ownership of their notes (no vendor lock-in)
- Anyone who maintains a daily log or journal

## Core Features

### 1. Note Management

- Create, read, edit, and delete notes
- List all notes with filtering options
- Open notes in your preferred editor (`$EDITOR`)

### 2. Daily Notes

- Quick command to open/create today's note
- Access yesterday's note or any specific date
- Automatic date-based file organization

### 3. Tags & Organization

- Add tags to notes via frontmatter or inline `#tags`
- Filter and list notes by tags
- Folder-based organization support

### 4. Search

- Full-text search across all notes
- Search by title, content, or tags
- Fast fuzzy matching

### 5. GitHub Sync

- Store notes in a Git repository
- Push/pull changes with simple commands
- Automatic commit messages
- Work offline, sync when ready

## Technical Requirements

| Requirement | Specification          |
| ----------- | ---------------------- |
| Language    | Rust                   |
| Note Format | Markdown (.md)         |
| Storage     | Local filesystem + Git |
| Sync        | GitHub (via git)       |
| Config      | TOML file              |
| Platforms   | macOS, Linux, Windows  |

## CLI Commands

```bash
noted                     # Open today's daily note
noted -c <new entry>      # Add a new entry to today's daily note
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

```
~/.noted/                 # Default notes directory (configurable)
├── .git/                 # Git repository
├── config.toml           # Configuration file
├── daily/                # Daily notes
│   └── 2026/
│       └── 01/
│           ├── 20.md     # Daily note for 2026-01-20
│           └── 21.md
├── notes/                # Regular notes
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

## Future Considerations

- **Templates**: Custom templates for different note types
- **Encryption**: Optional GPG encryption for sensitive notes
- **Export**: Export notes to PDF, HTML, or other formats
- **Hooks**: Pre/post save hooks for custom workflows
- **TUI mode**: Optional interactive terminal UI
- **Backlinks**: Wiki-style `[[note]]` linking between notes
- Allow to view tasks from notes

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

## Success Metrics

- App launches in < 100ms
- Note creation/opening in < 50ms
- Full-text search across 1000+ notes in < 500ms
- Zero data loss (Git provides history and backup)

## Development Phases

### Phase 1: Core MVP

- Basic note CRUD operations
- Daily notes functionality
- Local file storage

### Phase 2: Search & Organization

- Full-text search
- Tags support
- List filtering

### Phase 3: Git Integration

- Git init and commit
- Push/pull commands
- Sync workflow

### Phase 4: Polish

- Configuration system
- Templates
- Error handling and edge cases

## License

MIT
