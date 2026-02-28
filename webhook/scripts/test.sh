#!/bin/bash
# Webhook Talon - Test endpoint

URL="${1:-}"

if [ -z "$URL" ]; then
    echo "Usage: test.sh <url>"
    exit 1
fi

curl -s -o /dev/null -w "%{http_code}" "$URL"
