#!/bin/bash

# Usage check
if [ $# -ne 2 ]; then
    echo "Usage: $0 <test_directory> <file_without_extension> "
    exit 1
fi

TEST_DIRECTORY="$1"
NAME="$2"
SRC="${TEST_DIRECTORY}/${NAME}.rs"
BC_OUT="${NAME}.bc"
STRIPPED="smt_output/${NAME}_strip.bc"
SMT_OUT="smt_output/${NAME}.smt2"

echo "Compiling $SRC to LLVM bitcode..."

# Build Rust → LLVM bitcode (WORKING SETTINGS)
rustc +nightly-2022-03-01 \
    --emit=llvm-bc \
    -C overflow-checks=off \
    -C opt-level=0 \
    "$SRC" \
    -o "$BC_OUT"

if [ $? -ne 0 ]; then
    echo "❌ rustc failed"
    exit 1
fi

echo "✔ Built: $BC_OUT"

echo "Stripping debug symbols..."
opt-14 -strip-debug "$BC_OUT" -o "$STRIPPED"

echo "Running SeaHorn..."
seahorn --horn-no-verif --horn-format=smt2 \
    -o "$SMT_OUT" \
    "$STRIPPED"

echo "✔ Done: $SMT_OUT"
