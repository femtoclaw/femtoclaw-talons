---
name: sysinfo
version: 1.0.0
description: System information gathering - CPU, memory, disk, network
author: FemtoClaw Community
license: Apache-2.0
tags: [system, info, cpu, memory, monitoring, sysinfo]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Sysinfo Talon

Gathers system information including CPU, memory, disk, and network stats.

## What This Talon Does

Provides detailed system information for monitoring and diagnostics.

## Commands

### cpu
Gets CPU information.

**Arguments:**
- `detailed` (boolean, optional): Include detailed per-core stats

### memory
Gets memory usage information.

### disk
Gets disk usage information.

**Arguments:**
- `path` (string, optional): Path to check (default: all)

### network
Gets network interface information.

### processes
Lists running processes.

**Arguments:**
- `limit` (number, optional): Number of processes
- `sort_by` (string, optional): Sort by (cpu, memory, pid)

### system
Gets overall system information.

### uptime
Gets system uptime.

### hostname
Gets system hostname.

## Usage

```json
{"tool_call": {"tool": "sysinfo.cpu", "args": {}}}
```

```json
{"tool_call": {"tool": "sysinfo.memory", "args": {}}}
```

## Notes

- Uses sysinfo crate for cross-platform support
- Refreshes data on each call
- Supports sorting and filtering processes
