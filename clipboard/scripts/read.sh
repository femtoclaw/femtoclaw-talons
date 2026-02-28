#!/bin/bash
# Clipboard Talon - Read clipboard

if command -v xclip &> /dev/null; then
    xclip -selection clipboard -o
elif command -v pbcopy &> /dev/null; then
    pbpaste
else
    echo "Clipboard tool not found"
    exit 1
fi
