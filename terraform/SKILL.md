---
name: terraform
version: 1.0.0
description: Terraform Infrastructure as Code - plan, apply, manage cloud resources
author: FemtoClaw Community
license: Apache-2.0
tags: [terraform, iac, infrastructure, cloud, automation]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Terraform Skill

Provides Terraform Infrastructure as Code operations for managing cloud resources.

## What This Skill Does

Executes Terraform commands for infrastructure provisioning and management.

## Commands

### init
Initializes a Terraform working directory.

**Arguments:**
- `path` (string, optional): Terraform directory (default: current)
- `backend` (string, optional): Backend configuration

### plan
Generates and shows an execution plan.

**Arguments:**
- `path` (string, optional): Terraform directory
- `vars` (object, optional): Variable overrides
- `out` (string, optional): Output file path

### apply
Applies the Terraform changes.

**Arguments:**
- `path` (string, optional): Terraform directory
- `vars` (object, optional): Variable overrides
- `auto_approve` (boolean, optional): Skip approval prompt

### destroy
Destroys Terraform-managed infrastructure.

**Arguments:**
- `path` (string, optional): Terraform directory
- `auto_approve` (boolean, optional): Skip approval prompt

### validate
Validates Terraform configuration.

**Arguments:**
- `path` (string, optional): Terraform directory

### show
Shows current state or plan.

**Arguments:**
- `path` (string, optional): Terraform directory
- `state_file` (string, optional): State file path

### list_workspaces
Lists Terraform workspaces.

**Arguments:**
- `path` (string, optional): Terraform directory

### workspace_select
Switches to a workspace.

**Arguments:**
- `name` (string, required): Workspace name
- `path` (string, optional): Terraform directory

## Usage

```json
{"tool_call": {"tool": "terraform.plan", "args": {"path": "./infra"}}}
```

## Notes

- Requires terraform CLI installed
- Supports remote state backends
