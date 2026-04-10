#!/bin/bash
# Fetch Talon - HTTP GET request

URL="$1"

if [ -z "$URL" ]; then
    echo "Usage: get.sh <url>"
    exit 1
fi

curl -s "$URL"
