---
name: cron
version: 1.0.0
description: Cron job scheduling and management
author: FemtoClaw Community
license: Apache-2.0
tags: [cron, scheduler, job, automation, timer]
repository: https://github.com/femtoclaw/femtoclaw-skills
---

# Cron Skill

Provides cron job scheduling and management capabilities.

## What This Skill Does

Creates, lists, and manages scheduled cron jobs.

## Commands

### list
Lists all scheduled cron jobs.

### add
Adds a new cron job.

**Arguments:**
- `schedule` (string, required): Cron expression
- `command` (string, required): Command to run
- `description` (string, optional): Job description

### remove
Removes a cron job.

**Arguments:**
- `id` (string, required): Job ID

### enable
Enables a cron job.

**Arguments:**
- `id` (string, required): Job ID

### disable
Disables a cron job.

**Arguments:**
- `id` (string, required): Job ID

### run_now
Runs a cron job immediately.

**Arguments:**
- `id` (string, required): Job ID

### next_run
Shows next run time for a job.

**Arguments:**
- `schedule` (string, required): Cron expression

### validate
Validates a cron expression.

**Arguments:**
- `schedule` (string, required): Cron expression to validate

## Usage

```json
{"tool_call": {"tool": "cron.list", "args": {}}}
```

```json
{"tool_call": {"tool": "cron.add", "args": {"schedule": "0 * * * *", "command": "backup.sh", "description": "Hourly backup"}}}
```

## Notes

- Uses standard cron format: minute hour day month weekday
- Jobs stored in ~/.femtoclaw/cron/
- SupportsFemtoClaw command execution
