#!/bin/bash
# Scaffold Talon - List available patterns

SCAFFOLDS_DIR="${HOME}/.femtoclaw/scaffolds"

if [ -d "$SCAFFOLDS_DIR" ]; then
    echo "Available scaffold patterns:"
    ls -1 "$SCAFFOLDS_DIR"
else
    echo "No scaffolds found. Add patterns to ~/.femtoclaw/scaffolds/"
    exit 1
fi
