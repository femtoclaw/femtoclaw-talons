---
name: template
version: 1.0.0
description: General purpose template engine for text and file generation
author: FemtoClaw Community
license: Apache-2.0
tags: [template, text, generation, mustache, handlebars]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Template Skill

A general-purpose template engine for generating text and files from templates.

## What This Skill Does

Renders templates with variables for generating code, documents, and configurations.

## Commands

### render
Renders a template with provided variables.

**Arguments:**
- `template` (string, required): Template content or path
- `variables` (object, required): Variables for template
- `engine` (string, optional): Template engine (mustache, handlebars, jinja)

### render_file
Renders a template file to output.

**Arguments:**
- `input` (string, required): Input template file path
- `output` (string, required): Output file path
- `variables` (object, required): Variables for template

### list_engines
Lists supported template engines.

## Usage

```json
{"tool_call": {"tool": "template.render", "args": {"template": "Hello {{name}}!", "variables": {"name": "World"}}}}
```

## Notes

- Supports Mustache, Handlebars, and Jinja2-style templates
- Template files cached for performance
