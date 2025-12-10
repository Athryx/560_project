#!/bin/bash

# Usage:
# ./run_verus_batch.sh <directory>

if [ $# -lt 1 ]; then
    echo "Usage: $0 <directory>"
    exit 1
fi

DIR="$1"

# Check if directory exists
if [ ! -d "$DIR" ]; then
    echo "Error: Directory not found: $DIR"
    exit 1
fi

echo "Running Verus on all .rs files in directory: $DIR"
echo

# Enable nullglob so *.rs expands to nothing instead of literal string if no files
shopt -s nullglob

for FILE in "$DIR"/*.rs; do
    BASENAME=$(basename "$FILE")
    
    echo "==============================================="
    echo "🔍 Verifying: $BASENAME"
    echo "==============================================="

    # Run Verus (update command if needed)
    verus "$FILE"

    STATUS=$?

    if [ $STATUS -ne 0 ]; then
        echo "❌ Verus reported an error for $BASENAME"
    else
        echo "✔ Verus verification succeeded for $BASENAME"
    fi

    echo
done

echo "🏁 Finished processing all files."
