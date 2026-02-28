#!/bin/bash
# Matrix Talon - Send message

ROOM="${1:-}"
MESSAGE="${2:-}"

if [ -z "$ROOM" ] || [ -z "$MESSAGE" ]; then
    echo "Usage: send_message.sh <room> <message>"
    exit 1
fi

echo "Sending to $ROOM: $MESSAGE"
