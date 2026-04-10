#!/bin/bash
# Discord Talon - Send webhook message

WEBHOOK_URL="${1:-}"
MESSAGE="${2:-}"

if [ -z "$WEBHOOK_URL" ] || [ -z "$MESSAGE" ]; then
    echo "Usage: send.sh <webhook_url> <message>"
    exit 1
fi

curl -s -X POST "$WEBHOOK_URL" -H "Content-Type: application/json" -d "{\"content\": \"$MESSAGE\"}"
