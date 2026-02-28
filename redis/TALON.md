---
name: redis
version: 1.0.0
description: Redis operations - cache, strings, lists, sets, hashes
author: FemtoClaw Community
license: Apache-2.0
tags: [redis, cache, nosql, key-value, database]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# Redis Talon

Provides Redis operations for caching and key-value storage.

## What This Talon Does

Executes Redis commands for various data types and operations.

## Commands

### get
Gets a value by key.

**Arguments:**
- `key` (string, required): Redis key

### set
Sets a key-value pair.

**Arguments:**
- `key` (string, required): Redis key
- `value` (string, required): Value to store
- `ex` (number, optional): Expiration in seconds

### delete
Deletes one or more keys.

**Arguments:**
- `keys` (array, required): Keys to delete

### list_push
Pushes to a list.

**Arguments:**
- `key` (string, required): List key
- `value` (string, required): Value to push

### set_add
Adds to a set.

**Arguments:**
- `key` (string, required): Set key
- `members` (array, required): Members to add

### hash_set
Sets hash fields.

**Arguments:**
- `key` (string, required): Hash key
- `field` (string, required): Field name
- `value` (string, required): Field value

### keys
Lists keys matching a pattern.

**Arguments:**
- `pattern` (string, required): Key pattern

### exists
Checks if keys exist.

**Arguments:**
- `keys` (array, required): Keys to check

### expire
Sets expiration on a key.

**Arguments:**
- `key` (string, required): Key
- `seconds` (number, required): Expiration seconds

## Usage

```json
{"tool_call": {"tool": "redis.get", "args": {"key": "mykey"}}}
```

## Notes

- Requires REDIS_URL environment variable
- Supports Redis Cluster and Sentinel
