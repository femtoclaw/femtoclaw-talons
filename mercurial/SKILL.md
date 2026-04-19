---
name: mercurial
version: 1.0.0
description: Mercurial (Hg) distributed version control operations
author: FemtoClaw Community
license: Apache-2.0
tags: [mercurial, hg, version-control, vcs, distributed]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Mercurial Skill

Provides Mercurial (Hg) distributed version control operations.

## What This Skill Does

Executes Mercurial commands for distributed version control.

## Commands

### status
Shows working directory status.

**Arguments:**
- `path` (string, optional): Target path (default: current)

### commit
Commits changes to the repository.

**Arguments:**
- `message` (string, required): Commit message
- `path` (string, optional): Target path

### log
Shows commit history.

**Arguments:**
- `path` (string, required): Target path
- `limit` (number, optional): Number of entries

### branch
Shows or switches branches.

**Arguments:**
- `name` (string, optional): Branch name to switch to

### push
Pushes changes to remote.

**Arguments:**
- `path` (string, optional): Remote path or URL

### pull
Pulls changes from remote.

**Arguments:**
- `path` (string, optional): Remote path or URL

### diff
Shows differences between revisions.

**Arguments:**
- `rev1` (string, optional): First revision
- `rev2` (string, optional): Second revision

### clone
Clones a repository.

**Arguments:**
- `source` (string, required): Source repository URL
- `dest` (string, optional): Destination path

## Usage

```json
{"tool_call": {"tool": "mercurial.status", "args": {}}}
```

```json
{"tool_call": {"tool": "mercurial.commit", "args": {"message": "Add new feature"}}}
```

## Notes

- Requires hg command-line client installed
- Great for large repositories and efficient diffs
