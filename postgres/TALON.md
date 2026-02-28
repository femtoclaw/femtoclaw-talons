---
name: postgres
version: 1.0.0
description: PostgreSQL database operations
author: FemtoClaw Community
license: Apache-2.0
tags: [postgres, postgresql, database, sql]
repository: https://github.com/femtoclaw/femtoclaw-talons
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - database
environment:
  - name: PGHOST
    required: false
    description: PostgreSQL host
    default: localhost
  - name: PGPORT
    required: false
    description: PostgreSQL port
    default: "5432"
  - name: PGUSER
    required: true
    description: PostgreSQL user
  - name: PGPASSWORD
    required: true
    description: PostgreSQL password
  - name: PGDATABASE
    required: false
    description: Database name
commands:
  - name: query
    description: Execute SQL query
    args:
      - name: sql
        type: string
        required: true
        description: SQL query
  - name: tables
    description: List tables
    args: []
  - name: schema
    description: Get table schema
    args:
      - name: table
        type: string
        required: true
        description: Table name
  - name: backup
    description: Backup database
    args:
      - name: output
        type: string
        required: true
        description: Output file path
---

# PostgreSQL Talon

PostgreSQL database operations.

## Setup

```bash
export PGHOST=localhost
export PGPORT=5432
export PGUSER=myuser
export PGPASSWORD=mypassword
export PGDATABASE=mydb
```

## Usage

```json
{"tool_call": {"tool": "postgres.query", "args": {
  "sql": "SELECT * FROM users LIMIT 10"
}}}
```
