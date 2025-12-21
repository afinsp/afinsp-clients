#!/bin/bash

# Delete all generated files listed in codegen/generated_files.json

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
GENERATED_FILES_JSON="$ROOT_DIR/codegen/generated_files.json"

if [ ! -f "$GENERATED_FILES_JSON" ]; then
    echo "Error: $GENERATED_FILES_JSON not found"
    exit 1
fi

# Parse the JSON and delete each file
jq -r '.[][] // empty' "$GENERATED_FILES_JSON" | while read -r file; do
    filepath="$ROOT_DIR/$file"
    if [ -f "$filepath" ]; then
        rm "$filepath"
        echo "Deleted: $file"
    else
        echo "Not found: $file"
    fi
done

echo "Done."
