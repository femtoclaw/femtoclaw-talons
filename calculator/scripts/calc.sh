#!/bin/bash
# Calculator Talon - Simple calculator

EXPR="$1"

if [ -z "$EXPR" ]; then
    echo "Usage: calc.sh <expression>"
    echo "Example: calc.sh '2+2'"
    exit 1
fi

echo "$EXPR" | bc -l
