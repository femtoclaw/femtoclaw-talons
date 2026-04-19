---
name: mysql
version: 1.0.0
description: MySQL/MariaDB database operations
author: FemtoClaw Community
license: Apache-2.0
tags: [mysql, mariadb, database, sql]
repository: https://github.com/femtoclaw/femtoclaw-skills
runtime:
  kind: shell
  version: ">=1.0"
permissions:
  - shell
  - database
environment:
  - name: MYSQL_HOST
    required: false
    description: MySQL host
    default: localhost
  - name: MYSQL_PORT
    required: false
    description: MySQL port
    default: "3306"
  - name: MYSQL_USER
    required: true
    description: MySQL user
  - name: MYSQL_PASSWORD
    required: true
    description: MySQL password
  - name: MYSQL_DATABASE
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

# MySQL Skill

MySQL/MariaDB database operations.

## Setup

```bash
export MYSQL_HOST=localhost
export MYSQL_PORT=3306
export MYSQL_USER=myuser
export MYSQL_PASSWORD=mypassword
export MYSQL_DATABASE=mydb
```

## Usage

```json
{"tool_call": {"tool": "mysql.query", "args": {
  "sql": "SELECT * FROM users LIMIT 10"
}}}
```
