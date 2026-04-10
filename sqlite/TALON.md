---
name: sqlite
version: 1.0.0
description: SQLite database operations - queries, migrations, backups
author: FemtoClaw Community
license: Apache-2.0
tags: [sqlite, database, sql, embedded, local]
repository: https://github.com/femtoclaw/femtoclaw-talons
---

# SQLite Talon

Provides SQLite database operations for embedded databases.

## What This Talon Does

Executes SQL queries and manages SQLite databases.

## Commands

### execute
Executes a SQL statement.

**Arguments:**
- `sql` (string, required): SQL statement
- `database` (string, optional): Database path

### query
Executes a SELECT query and returns results.

**Arguments:**
- `sql` (string, required): SELECT statement
- `database` (string, optional): Database path

### create_table
Creates a new table.

**Arguments:**
- `table` (string, required): Table name
- `schema` (string, required): CREATE TABLE SQL
- `database` (string, optional): Database path

### list_tables
Lists all tables in database.

**Arguments:**
- `database` (string, optional): Database path

### backup
Backs up a database.

**Arguments:**
- `source` (string, required): Source database path
- `destination` (string, required): Backup destination path

### vacuum
Optimizes database file.

**Arguments:**
- `database` (string, required): Database path

### dump
Dumps database to SQL file.

**Arguments:**
- `database` (string, required): Database path
- `output` (string, required): Output SQL file

## Usage

```json
{"tool_call": {"tool": "sqlite.query", "args": {"sql": "SELECT * FROM users LIMIT 10"}}}
```

## Notes

- Works with local .db files
- Supports in-memory databases
- Full ACID compliance
