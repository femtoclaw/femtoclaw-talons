#!/bin/bash
# Database Talon - List tables

DB="$1"

if [ -z "$DB" ]; then
    echo "Usage: list_tables.sh <database>"
    exit 1
fi

sqlite3 "$DB" ".tables"
