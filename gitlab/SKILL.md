---
name: gitlab
version: 1.0.0
description: GitLab integration for issues, merge requests, and pipelines
author: FemtoClaw Community
license: Apache-2.0
tags: [gitlab, version-control, devops, ci-cd]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - network
environment:
  - name: GITLAB_TOKEN
    required: true
    description: GitLab personal access token
  - name: GITLAB_URL
    required: false
    description: GitLab instance URL
    default: https://gitlab.com
commands:
  - name: list_issues
    description: List GitLab issues
    args:
      - name: project
        type: string
        required: true
        description: Project path (namespace/project)
      - name: state
        type: string
        required: false
        description: Issue state (opened/closed/all)
        default: opened
  - name: create_issue
    description: Create a GitLab issue
    args:
      - name: project
        type: string
        required: true
        description: Project path
      - name: title
        type: string
        required: true
        description: Issue title
      - name: description
        type: string
        required: false
        description: Issue description
  - name: list_mrs
    description: List merge requests
    args:
      - name: project
        type: string
        required: true
        description: Project path
      - name: state
        type: string
        required: false
        description: MR state
        default: opened
  - name: trigger_pipeline
    description: Trigger a CI/CD pipeline
    args:
      - name: project
        type: string
        required: true
        description: Project path
      - name: ref
        type: string
        required: false
        description: Git ref
        default: main
---

# GitLab Skill

Provides GitLab integration for issues, merge requests, and pipelines.

## Requirements

### GitLab Token

Create a personal access token at:
https://gitlab.com/-/profile/personal_access_tokens

Required scopes:
- `api` - Full API access
- `read_api` - Read API access

Set the token:

```bash
export GITLAB_TOKEN=your-token-here
```

## Commands

### list_issues

```json
{"tool_call": {"tool": "gitlab.list_issues", "args": {"project": "namespace/project"}}}
```

### create_issue

```json
{"tool_call": {"tool": "gitlab.create_issue", "args": {
  "project": "namespace/project",
  "title": "New Issue",
  "description": "Issue description"
}}}
```
