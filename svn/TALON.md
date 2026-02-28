---
name: svn
version: 1.0.0
description: Subversion (SVN) version control operations
author: FemtoClaw Community
license: Apache-2.0
tags: [svn, version-control, vcs, subversion]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# SVN Talon

Provides Subversion (SVN) version control operations.

## What This Talon Does

Executes SVN commands for version control operations.

## Commands

### status
Shows working copy status.

**Arguments:**
- `path` (string, optional): Target path (default: current)

### commit
Commits changes to the repository.

**Arguments:**
- `message` (string, required): Commit message
- `path` (string, optional): Target path

### update
Updates working copy from repository.

**Arguments:**
- `path` (string, optional): Target path
- `revision` (string, optional): Specific revision

### log
Shows commit log.

**Arguments:**
- `path` (string, required): Target path
- `limit` (number, optional): Number of entries

### diff
Shows differences between versions.

**Arguments:**
- `path` (string, required): Target path
- `revision1` (string, optional): First revision
- `revision2` (string, optional): Second revision

### add
Adds files to version control.

**Arguments:**
- `path` (string, required): File or directory path

### switch
Switches working copy to a different branch.

**Arguments:**
- `url` (string, required): Repository URL
- `path` (string, optional): Working copy path

## Usage

```json
{"tool_call": {"tool": "svn.status", "args": {}}}
```

```json
{"tool_call": {"tool": "svn.commit", "args": {"message": "Fix bug in login"}}}
```

## Notes

- Requires svn command-line client installed
- Supports svnserve and HTTP(S) repositories
