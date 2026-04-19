---
name: starter
version: 1.0.0
description: A starter template skill for quickly creating new skills
author: FemtoClaw Community
license: Apache-2.0
tags: [template, starter, example]
repository: https://github.com/femtoclaw/femtoclaw-skills
commands:
  - name: init
    description: Initialize a new skill from template
    args:
      - name: name
        type: string
        required: true
        description: Name of new skill
---

# Starter Skill

A minimal starter template for quickly creating new Skills.

## Usage

```bash
skill add ./femtoclaw-skills/starter
```

## Purpose

This skill serves as a minimal starting point for creating new Skills.
Copy it and customize the SKILL.md file.
