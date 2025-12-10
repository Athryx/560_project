#!/bin/bash

# Usage:
# ./run_batch.sh <directory> <learning-type> [--with-smt2] [--rust-only] [--merge <n>] [--annotated]

if [ $# -lt 2 ]; then
    echo "Usage: $0 <directory> <learning-type> [optional flags]"
    echo "Optional flags: --with-smt2 --rust-only --merge <n> --annotated"
    exit 1
fi

DIR="$1"
LEARNING_TYPE="$2"
shift 2  # remove required args so $@ now contains only optionals

# Store optional args for forwarding to main.py
OPTIONAL_ARGS=()

# Track whether annotated flag is present
ANNOTATED_FLAG=false

# Parse remaining arguments
while [[ $# -gt 0 ]]; do
    case "$1" in
        --with-smt2)
            OPTIONAL_ARGS+=("--with-smt2")
            shift
            ;;
        --rust-only)
            OPTIONAL_ARGS+=("--rust-only")
            shift
            ;;
        --annotated)
            OPTIONAL_ARGS+=("--annotated")
            ANNOTATED_FLAG=true
            shift
            ;;
        --merge)
            OPTIONAL_ARGS+=("--merge" "$2")
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Iterate through all .rs files inside the directory
for FILE in "$DIR"/*.rs; do
    if [ -f "$FILE" ]; then
        BASENAME=$(basename "$FILE")

        # Determine output file based on annotation flag
        if [ "$ANNOTATED_FLAG" = true ]; then
            OUTFILE="${BASENAME%.rs}_verified_annotated.rs"
        else
            OUTFILE="${BASENAME%.rs}_verified.rs"
        fi

        echo "==========================================="
        echo "Processing: $FILE"
        echo "Output will be: $OUTFILE"
        echo "Using extra flags: ${OPTIONAL_ARGS[*]}"
        echo "==========================================="

        python code/main.py \
            --input "$FILE" \
            --output "$OUTFILE" \
            --config code/config-artifact-openai.json \
            --learning-type "$LEARNING_TYPE" \
            "${OPTIONAL_ARGS[@]}"
    fi
done
