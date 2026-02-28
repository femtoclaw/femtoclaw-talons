---
name: scaffold
version: 1.0.0
description: Code scaffolding for generating file structures and patterns
author: FemtoClaw Community
license: Apache-2.0
tags: [scaffold, code-gen, generator, pattern]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Scaffold Talon

Generates code files and directory structures from scaffold patterns.

## What This Talon Does

Scaffold creates new code files, directories, and patterns based on predefined templates.

## Commands

### generate
Generates code from a scaffold pattern.

**Arguments:**
- `pattern` (string, required): Scaffold pattern name
- `name` (string, required): Name for generated files
- `options` (object, optional): Additional pattern options

### list_patterns
Lists available scaffold patterns.

### init
Initializes scaffold in the current directory.

**Arguments:**
- `template` (string, optional): Template to use

## Usage

```json
{"tool_call": {"tool": "scaffold.generate", "args": {"pattern": "rust-crate", "name": "my-crate"}}}
```

## Notes

- Patterns defined in ~/.femtoclaw/scaffolds/
- Supports custom pattern definitions
