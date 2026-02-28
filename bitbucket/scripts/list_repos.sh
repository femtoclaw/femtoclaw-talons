#!/bin/bash
# Bitbucket Talon - List repositories

WORKSPACE="${1:-}"

if [ -z "$WORKSPACE" ]; then
    echo "Usage: list_repos.sh <workspace>"
    exit 1
fi

echo "Fetching repositories for workspace: $WORKSPACE"
echo "Use femtoclaw to call: bitbucket.list_repos {\"workspace\": \"$WORKSPACE\"}"
