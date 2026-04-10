---
name: calculator
version: 1.0.0
description: Calculator with basic and scientific operations
author: FemtoClaw Community
license: Apache-2.0
tags: [calculator, math, utilities]
repository: https://github.com/femtoclaw/femtoclaw-talons
commands:
  - name: add
    description: Addition
    args:
      - name: a
        type: number
        required: true
      - name: b
        type: number
        required: true
  - name: subtract
    description: Subtraction
    args:
      - name: a
        type: number
        required: true
      - name: b
        type: number
        required: true
  - name: multiply
    description: Multiplication
    args:
      - name: a
        type: number
        required: true
      - name: b
        type: number
        required: true
  - name: divide
    description: Division
    args:
      - name: a
        type: number
        required: true
      - name: b
        type: number
        required: true
  - name: power
    description: Exponentiation
    args:
      - name: base
        type: number
        required: true
      - name: exponent
        type: number
        required: true
  - name: sqrt
    description: Square root
    args:
      - name: value
        type: number
        required: true
  - name: eval
    description: Evaluate mathematical expression
    args:
      - name: expression
        type: string
        required: true
        description: Mathematical expression (e.g., "2+2*3")
---

# Calculator Talon

Basic and scientific calculator operations.

## Usage

```json
{"tool_call": {"tool": "calculator.add", "args": {"a": 10, "b": 5}}}
{"tool_call": {"tool": "calculator.sqrt", "args": {"value": 16}}}
{"tool_call": {"tool": "calculator.eval", "args": {"expression": "(2+3)*4"}}}
```
