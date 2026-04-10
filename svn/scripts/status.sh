#!/bin/bash
# SVN Talon - Show status

PATH="${1:-.}"

echo "Checking SVN status for: $PATH"
svn status "$PATH"
