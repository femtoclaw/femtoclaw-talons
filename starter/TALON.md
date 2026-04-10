---
name: starter
version: 1.0.0
description: A starter template talon for quickly creating new talons
author: FemtoClaw Community
license: Apache-2.0
tags: [template, starter, example]
repository: https://github.com/femtoclaw/femtoclaw-talons
commands:
  - name: init
    description: Initialize a new talon from template
    args:
      - name: name
        type: string
        required: true
        description: Name of new talon
---

# Starter Talon

A minimal starter template for quickly creating new Talons.

## Usage

```bash
talon add ./femtoclaw-talons/starter
```

## Purpose

This talon serves as a minimal starting point for creating new Talons.
Copy it and customize the TALON.md file.
