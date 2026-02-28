#!/bin/bash
# Boilerplate Talon - List available templates

TEMPLATES_DIR="${HOME}/.femtoclaw/boilerplates"

if [ -d "$TEMPLATES_DIR" ]; then
    echo "Available boilerplates:"
    ls -1 "$TEMPLATES_DIR"
else
    echo "No boilerplates found. Add templates to ~/.femtoclaw/boilerplates/"
    exit 1
fi
