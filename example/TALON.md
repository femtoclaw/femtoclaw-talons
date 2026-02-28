---
name: example
version: 1.0.0
description: An example talon demonstrating the TALON.md format
author: FemtoClaw Community
license: Apache-2.0
tags: [example, demo, tutorial]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Example Talon

This is an example talon that demonstrates the TALON.md format and structure.

## What This Talon Does

This talon provides a simple example of how to create a Talon for FemtoClaw.
It serves as a template for creating new Talons.

## Commands

### greet
Greets the user with a customizable message.

**Arguments:**
- `name` (string, required): Name to greet
- `enthusiastic` (boolean, optional): Add exclamation marks

### echo
Echoes back the input text.

**Arguments:**
- `text` (string, required): Text to echo

## Requirements

- None - This is a simple example talon

## Usage

This talon can be used as a starting point for creating your own talons.
Simply copy this directory and modify the TALON.md file.

## Example Input

```json
{"tool_call": {"tool": "example.greet", "args": {"name": "World"}}}
```

## Output

```json
{"message": {"content": "Hello, World!"}}
```

## Notes

- This talon is meant as a learning tool
- Modify as needed for your use case
- See femtoclaw-talons/README.md for more information
