#!/bin/bash
# Boilerplate Talon - Create project from template

TEMPLATE="$1"
PROJECT_NAME="$2"
DEST_DIR="${3:-.}"

TEMPLATES_DIR="${HOME}/.femtoclaw/boilerplates"

if [ -z "$TEMPLATE" ] || [ -z "$PROJECT_NAME" ]; then
    echo "Usage: create.sh <template> <project_name> [destination]"
    exit 1
fi

TEMPLATE_PATH="$TEMPLATES_DIR/$TEMPLATE"

if [ ! -d "$TEMPLATE_PATH" ]; then
    echo "Template '$TEMPLATE' not found"
    exit 1
fi

cp -r "$TEMPLATE_PATH" "$DEST_DIR/$PROJECT_NAME"
echo "Created project '$PROJECT_NAME' from template '$TEMPLATE'"
