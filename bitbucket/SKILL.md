---
name: bitbucket
version: 1.0.0
description: Bitbucket Cloud integration for repositories, pull requests, and pipelines
author: FemtoClaw Community
license: Apache-2.0
tags: [bitbucket, vcs, git, repository, pipelines]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Bitbucket Skill

Integrates with Bitbucket Cloud for repository management and CI/CD pipelines.

## What This Skill Does

Provides commands to manage Bitbucket repositories, pull requests, and pipelines.

## Commands

### list_repos
Lists repositories in a workspace.

**Arguments:**
- `workspace` (string, required): Bitbucket workspace name

### create_repo
Creates a new repository.

**Arguments:**
- `name` (string, required): Repository name
- `workspace` (string, required): Workspace name
- `is_private` (boolean, optional): Make repository private

### list_pulls
Lists pull requests in a repository.

**Arguments:**
- `repo` (string, required): Repository slug
- `workspace` (string, required): Workspace name
- `state` (string, optional): Filter by state (OPEN, MERGED, CLOSED)

### create_pull
Creates a new pull request.

**Arguments:**
- `title` (string, required): PR title
- `source` (string, required): Source branch
- `target` (string, required): Target branch
- `repo` (string, required): Repository name

### trigger_pipeline
Triggers a Bitbucket pipeline.

**Arguments:**
- `repo` (string, required): Repository name
- `branch` (string, optional): Branch to run
- `pipeline` (string, optional): Pipeline name

## Usage

```json
{"tool_call": {"tool": "bitbucket.list_repos", "args": {"workspace": "my-team"}}}
```

## Notes

- Requires BITBUCKET_USERNAME and BITBUCKET_APP_PASSWORD environment variables
- Supports workspace-level permissions
