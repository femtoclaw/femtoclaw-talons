#!/bin/bash
# SQLite Talon - List tables

DATABASE="${1:-}"

if [ -z "$DATABASE" ]; then
    echo "Usage: list_tables.sh <database>"
    exit 1
fi

sqlite3 "$DATABASE" ".tables"
