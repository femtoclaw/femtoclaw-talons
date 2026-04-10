#!/bin/bash
# Redis Talon - Get value

KEY="${1:-}"

if [ -z "$KEY" ]; then
    echo "Usage: get.sh <key>"
    exit 1
fi

redis-cli GET "$KEY"
