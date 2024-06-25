#!/bin/bash

# Check if the out directory argument is provided
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <out-directory> <source-directory>"
    exit 1
fi

# Set the out directory and source directory from the arguments
OUT_DIR=$1
SOURCE_DIR=$2

# Create the out directory
mkdir -p "$OUT_DIR"

# Clone the repository
git clone https://github.com/kyclark/command-line-rust.git

# Copy the files from the specific directory
cp -r "command-line-rust/$SOURCE_DIR/"* "$OUT_DIR/"

# Clean up
rm -rf command-line-rust

echo "Files have been copied to $OUT_DIR from $SOURCE_DIR"

