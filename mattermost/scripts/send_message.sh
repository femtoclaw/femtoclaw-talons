#!/bin/bash
# Mattermost Talon - Send message

CHANNEL="${1:-}"
MESSAGE="${2:-}"

if [ -z "$CHANNEL" ] || [ -z "$MESSAGE" ]; then
    echo "Usage: send_message.sh <channel> <message>"
    exit 1
fi

echo "Sending to $CHANNEL: $MESSAGE"
