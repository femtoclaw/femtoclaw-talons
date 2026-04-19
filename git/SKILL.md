---
name: git
version: 1.0.0
description: Git operations - commit, push, pull, branch, and more
author: FemtoClaw Community
license: Apache-2.0
tags: [git, version-control, devtools, source-code]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
commands:
  - name: status
    description: Show git status
    args: []
  - name: add
    description: Stage files for commit
    args:
      - name: files
        type: string
        required: false
        description: Files to stage (default: all)
        default: "."
  - name: commit
    description: Commit staged changes
    args:
      - name: message
        type: string
        required: true
        description: Commit message
  - name: push
    description: Push commits to remote
    args:
      - name: remote
        type: string
        required: false
        description: Remote name
        default: origin
      - name: branch
        type: string
        required: false
        description: Branch name
  - name: pull
    description: Pull from remote
    args:
      - name: remote
        type: string
        required: false
        description: Remote name
        default: origin
      - name: branch
        type: string
        required: false
        description: Branch name
  - name: branch_list
    description: List branches
    args:
      - name: all
        type: boolean
        required: false
        description: Show all branches
        default: false
  - name: branch_create
    description: Create a new branch
    args:
      - name: name
        type: string
        required: true
        description: Branch name
      - name: start_point
        type: string
        required: false
        description: Starting commit/branch
  - name: log
    description: Show commit history
    args:
      - name: limit
        type: integer
        required: false
        description: Number of commits
        default: 10
  - name: diff
    description: Show changes
    args:
      - name: target
        type: string
        required: false
        description: Compare target (branch/commit)
---

# Git Skill

Provides Git version control operations for FemtoClaw.

## Features

- **Repository Operations**: Status, commit, push, pull
- **Branch Management**: Create, list, switch branches
- **History**: View commit logs and diffs
- **Staging**: Add and manage file staging

## Requirements

### Git

```bash
git --version
```

### Configuration

Configure git if needed:

```bash
git config --global user.name "Your Name"
git config --global user.email "your@email.com"
```

## Commands

### status

Show working tree status:

```bash
git status
```

### commit

Commit staged changes:

```bash
git commit -m "Your message"
```

### push

Push to remote:

```bash
git push origin main
```

### branch_create

Create a new branch:

```bash
git checkout -b feature/my-feature
```

## Use Cases

- Automated commits
- CI/CD workflows
- Repository management
- Code review automation
