---
name: demo
version: 1.0.0
description: A demonstration skill showcasing various capability patterns
author: FemtoClaw Community
license: Apache-2.0
tags: [demo, demonstration, tutorial]
repository: https://github.com/femtoclaw/femtoclaw-skills
commands:
  - name: hello
    description: Simple hello world command
    args:
      - name: name
        type: string
        required: false
        description: Name to greet
        default: World
  - name: echo
    description: Echo back input
    args:
      - name: text
        type: string
        required: true
        description: Text to echo
  - name: add
    description: Add two numbers
    args:
      - name: a
        type: number
        required: true
        description: First number
      - name: b
        type: number
        required: true
        description: Second number
---

# Demo Skill

A demonstration skill showcasing various command patterns.

## Examples

```json
{"tool_call": {"tool": "demo.hello", "args": {"name": "FemtoClaw"}}}
{"tool_call": {"tool": "demo.add", "args": {"a": 5, "b": 3}}}
```
