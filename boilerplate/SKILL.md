---
name: boilerplate
version: 1.0.0
description: Pre-made project boilerplate templates for quick project starts
author: FemtoClaw Community
license: Apache-2.0
tags: [template, boilerplate, project, scaffolding]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Boilerplate Skill

Provides pre-made project boilerplate templates for quick project starts.

## What This Skill Does

Create new projects from pre-built boilerplate templates for various languages and frameworks.

## Commands

### list
Lists available boilerplate templates.

### create
Creates a new project from a boilerplate template.

**Arguments:**
- `template` (string, required): Template name to use
- `project_name` (string, required): Name for the new project
- `directory` (string, optional): Target directory

### add_template
Adds a custom boilerplate template to the registry.

**Arguments:**
- `name` (string, required): Template name
- `path` (string, required): Path to template directory

## Usage

```json
{"tool_call": {"tool": "boilerplate.list", "args": {}}}
```

```json
{"tool_call": {"tool": "boilerplate.create", "args": {"template": "rust-cli", "project_name": "my-app"}}}
```

## Notes

- Templates are stored in ~/.femtoclaw/boilerplates/
- Supports custom template paths
