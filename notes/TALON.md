---
name: notes
version: 1.0.0
description: Simple note-taking and organization
author: FemtoClaw Community
license: Apache-2.0
tags: [notes, productivity, todo, organization]
repository: https://github.com/femtoclaw/femtoclaw-talons
environment:
  - name: NOTES_DIR
    required: false
    description: Directory for notes
    default: ./notes
commands:
  - name: create
    description: Create a new note
    args:
      - name: title
        type: string
        required: true
        description: Note title
      - name: content
        type: string
        required: true
        description: Note content
  - name: list
    description: List all notes
    args: []
  - name: read
    description: Read a note
    args:
      - name: title
        type: string
        required: true
        description: Note title
  - name: delete
    description: Delete a note
    args:
      - name: title
        type: string
        required: true
        description: Note title
  - name: append
    description: Append to a note
    args:
      - name: title
        type: string
        required: true
        description: Note title
      - name: content
        type: string
        required: true
        description: Content to append
---

# Notes Talon

Simple note-taking and organization.

## Features

- Create, read, delete notes
- List all notes
- Append to existing notes

## Usage

### Create note

```json
{"tool_call": {"tool": "notes.create", "args": {
  "title": "ideas",
  "content": "Build an AI agent"
}}}
```

### List notes

```json
{"tool_call": {"tool": "notes.list", "args": {}}}
```

### Read note

```json
{"tool_call": {"tool": "notes.read", "args": {
  "title": "ideas"
}}}
```

## Storage

Notes are stored as Markdown files in the configured directory.
