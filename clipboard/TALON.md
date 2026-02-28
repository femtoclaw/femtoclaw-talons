---
name: clipboard
version: 1.0.0
description: Clipboard management - read, write, and history
author: FemtoClaw Community
license: Apache-2.0
tags: [clipboard, copy, paste, system, utility]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Clipboard Talon

Provides clipboard read, write, and history management.

## What This Talon Does

Manages system clipboard with history tracking and text formatting.

## Commands

### read
Reads current clipboard content.

### write
Writes text to clipboard.

**Arguments:**
- `text` (string, required): Text to copy

### history
Shows clipboard history.

**Arguments:**
- `limit` (number, optional): Number of items (default: 10)

### clear
Clears the clipboard.

### copy_file
Copies a file to clipboard (file path).

**Arguments:**
- `path` (string, required): File path

### paste_image
Pastes image from clipboard to file.

**Arguments:**
- `output_path` (string, required): Output file path

## Usage

```json
{"tool_call": {"tool": "clipboard.read", "args": {}}}
```

```json
{"tool_call": {"tool": "clipboard.write", "args": {"text": "Hello, World!"}}}
```

## Notes

- Maintains configurable history (default: 50 items)
- Supports text and image content
- Platform-specific implementations for Windows, macOS, Linux
