---
name: github
version: 1.0.0
description: GitHub integration for issues, pull requests, and workflows
author: FemtoClaw Community
license: Apache-2.0
tags: [github, devtools, automation, git]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: GH_TOKEN
    required: true
    description: GitHub personal access token
commands:
  - name: list_issues
    description: List GitHub issues in a repository
    args:
      - name: owner
        type: string
        required: true
        description: Repository owner
      - name: repo
        type: string
        required: true
        description: Repository name
      - name: state
        type: string
        required: false
        description: Issue state (open/closed/all)
        default: open
  - name: create_issue
    description: Create a new GitHub issue
    args:
      - name: owner
        type: string
        required: true
        description: Repository owner
      - name: repo
        type: string
        required: true
        description: Repository name
      - name: title
        type: string
        required: true
        description: Issue title
      - name: body
        type: string
        required: false
        description: Issue body
  - name: list_prs
    description: List pull requests
    args:
      - name: owner
        type: string
        required: true
        description: Repository owner
      - name: repo
        type: string
        required: true
        description: Repository name
      - name: state
        type: string
        required: false
        description: PR state (open/closed/all)
        default: open
  - name: run_workflow
    description: Trigger a GitHub Actions workflow
    args:
      - name: owner
        type: string
        required: true
        description: Repository owner
      - name: repo
        type: string
        required: true
        description: Repository name
      - name: workflow
        type: string
        required: true
        description: Workflow file name or ID
      - name: ref
        type: string
        required: false
        description: Git ref (branch/tag)
        default: main
---

# GitHub Talon

Provides GitHub integration for FemtoClaw, enabling interaction with GitHub repositories.

## Features

- **Issue Management**: Create, list, and manage GitHub issues
- **Pull Requests**: List and manage pull requests
- **Workflows**: Trigger GitHub Actions workflows
- **Repository Info**: Get repository information

## Requirements

### GitHub CLI (gh)

This talon requires the GitHub CLI to be installed:

```bash
# macOS
brew install gh

# Linux
brew install gh

# Windows
winget install GitHub.cli
```

### Authentication

Set the `GH_TOKEN` environment variable with a GitHub personal access token:

```bash
export GH_TOKEN=ghp_your_token_here
```

Create a token at: https://github.com/settings/tokens

Required scopes:
- `repo` (for private repos)
- `public_repo` (for public repos only)

## Commands

### list_issues

List issues in a repository:

```bash
gh issue list --owner owner --repo repo --state open
```

### create_issue

Create a new issue:

```bash
gh issue create --owner owner --repo repo --title "Issue title" --body "Issue body"
```

### list_prs

List pull requests:

```bash
gh pr list --owner owner --repo repo --state open
```

### run_workflow

Trigger a workflow:

```bash
gh workflow run workflow.yml --ref main
```

## Security Considerations

- **Token Storage**: Store `GH_TOKEN` securely, never commit it
- **Scope**: Use minimum required scopes for your use case
- **Permissions**: Review talon permissions before installing

## Example Usage

```json
{"tool_call": {"tool": "github.list_issues", "args": {"owner": "femtoclaw", "repo": "femtoclaw"}}}
```

## Notes

- This talon uses the `gh` CLI for GitHub operations
- Ensure `gh` is authenticated with `gh auth login`
- Some commands require appropriate repository permissions
