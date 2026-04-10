#!/bin/bash
# Email Talon - Send test email

TO="$1"
SUBJECT="$2"
BODY="$3"

if [ -z "$TO" ] || [ -z "$SUBJECT" ]; then
    echo "Usage: send.sh <to> <subject> [body]"
    exit 1
fi

echo "$BODY" | mail -s "$SUBJECT" "$TO"
