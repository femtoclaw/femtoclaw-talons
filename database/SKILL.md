---
name: database
version: 1.0.0
description: Database operations - query, backup, and manage databases
author: FemtoClaw Community
license: Apache-2.0
tags: [database, sql, postgres, mysql, devops]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - database
environment:
  - name: DATABASE_URL
    required: true
    description: Database connection string
commands:
  - name: query
    description: Execute a SQL query
    args:
      - name: sql
        type: string
        required: true
        description: SQL query to execute
  - name: tables
    description: List all tables
    args: []
  - name: schema
    description: Get table schema
    args:
      - name: table
        type: string
        required: true
        description: Table name
  - name: backup
    description: Create database backup
    args:
      - name: output
        type: string
        required: true
        description: Output file path
  - name: restore
    description: Restore database from backup
    args:
      - name: backup_file
        type: string
        required: true
        description: Backup file path
  - name: ping
    description: Test database connection
    args: []
---

# Database Skill

Provides database operations for FemtoClaw - query, backup, and manage.

## Features

- **SQL Execution**: Run arbitrary SQL queries
- **Schema Inspection**: View tables and schemas
- **Backup/Restore**: Database backup and restore
- **Connection Testing**: Verify database connectivity

## Requirements

### Database Client

Install appropriate client:

```bash
# PostgreSQL
brew install postgresql

# MySQL
brew install mysql-client
```

### Connection

Set the `DATABASE_URL` environment variable:

```bash
# PostgreSQL
export DATABASE_URL=postgres://user:pass@localhost/mydb

# MySQL
export DATABASE_URL=mysql://user:pass@localhost/mydb

# SQLite
export DATABASE_URL=sqlite:///path/to/db.sqlite
```

## Commands

### query

Execute a query:

```json
{"tool_call": {"tool": "database.query", "args": {
  "sql": "SELECT * FROM users LIMIT 10"
}}}
```

### tables

List all tables:

```bash
# PostgreSQL
psql -c "\dt"

# MySQL
SHOW TABLES;
```

### schema

Get table schema:

```bash
# PostgreSQL
\d+ table_name

# MySQL
DESCRIBE table_name;
```

### backup

Create a backup:

```bash
pg_dump $DATABASE_URL > backup.sql
```

## Use Cases

- Database administration
- Data migration
- Schema inspection
- Automated backups
- Data analysis

## Security

- Never log credentials
- Use connection pooling
- Validate SQL inputs
- Limit query permissions
- Audit sensitive operations

## Supported Databases

- PostgreSQL
- MySQL/MariaDB
- SQLite
- (Extensible for others)
