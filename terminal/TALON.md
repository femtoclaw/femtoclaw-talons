---
name: terminal
version: 1.0.0
description: Terminal utilities - process management, file operations, system info
author: FemtoClaw Community
license: Apache-2.0
tags: [terminal, shell, system, devtools, utilities]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
commands:
  - name: run
    description: Run a shell command
    args:
      - name: command
        type: string
        required: true
        description: Command to execute
      - name: cwd
        type: string
        required: false
        description: Working directory
  - name: processes
    description: List running processes
    args:
      - name: limit
        type: integer
        required: false
        description: Number of processes
        default: 20
  - name: disk_usage
    description: Show disk usage
    args:
      - name: path
        type: string
        required: false
        description: Path to check
        default: .
  - name: memory
    description: Show memory usage
    args: []
  - name: cpu
    description: Show CPU usage
    args: []
  - name: uptime
    description: Show system uptime
    args: []
  - name: find_file
    description: Find files by name
    args:
      - name: pattern
        type: string
        required: true
        description: File name pattern
      - name: path
        type: string
        required: false
        description: Search path
        default: .
  - name: read_file
    description: Read file contents
    args:
      - name: path
        type: string
        required: true
        description: File path
      - name: lines
        type: integer
        required: false
        description: Number of lines
        default: 100
---

# Terminal Talon

Provides essential terminal and system operations for FemtoClaw.

## Features

- **Command Execution**: Run arbitrary shell commands
- **Process Management**: List and manage processes
- **System Monitoring**: CPU, memory, disk usage
- **File Operations**: Read, find, manage files
- **System Info**: Uptime, system details

## Requirements

### Shell Access

This talon requires shell access. It works with:
- bash
- zsh
- fish
- PowerShell

## Commands

### run

Execute a shell command:

```json
{"tool_call": {"tool": "terminal.run", "args": {
  "command": "ls -la"
}}}
```

### processes

List running processes:

```bash
ps aux | head -20
```

### disk_usage

Check disk usage:

```bash
df -h .
```

### memory

Show memory usage:

```bash
free -h
```

### cpu

Show CPU usage:

```bash
top -bn1 | head -5
```

### find_file

Find files:

```json
{"tool_call": {"tool": "terminal.find_file", "args": {
  "pattern": "*.rs",
  "path": "./src"
}}}
```

### read_file

Read file contents:

```json
{"tool_call": {"tool": "terminal.read_file", "args": {
  "path": "./Cargo.toml",
  "lines": 50
}}}
```

## Use Cases

- System monitoring and alerts
- File management automation
- Process investigation
- Development workflows
- DevOps automation

## Security Considerations

- **Caution**: This talon can execute arbitrary commands
- Validate all user inputs
- Limit available commands in production
- Use sandboxed execution
- Audit command execution
- Never run untrusted commands

## Quick Examples

### Check disk space

```bash
df -h
```

### Find large files

```bash
find . -type f -size +100M
```

### Kill a process

```bash
kill -9 <pid>
```

### Check listening ports

```bash
netstat -tlnp
```
