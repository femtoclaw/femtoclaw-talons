#!/bin/bash
# Mercurial Talon - Show status

PATH="${1:-.}"

echo "Checking Mercurial status for: $PATH"
hg status "$PATH"
