---
name: example
version: 1.0.0
description: An example skill demonstrating the SKILL.md format
author: FemtoClaw Community
license: Apache-2.0
tags: [example, demo, tutorial]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Example Skill

This is an example skill that demonstrates the SKILL.md format and structure.

## What This Skill Does

This skill provides a simple example of how to create a Skill for FemtoClaw.
It serves as a template for creating new Skills.

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

- None - This is a simple example skill

## Usage

This skill can be used as a starting point for creating your own skills.
Simply copy this directory and modify the SKILL.md file.

## Example Input

```json
{"tool_call": {"tool": "example.greet", "args": {"name": "World"}}}
```

## Output

```json
{"message": {"content": "Hello, World!"}}
```

## Notes

- This skill is meant as a learning tool
- Modify as needed for your use case
- See femtoclaw-skills/README.md for more information
