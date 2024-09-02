#!/bin/bash
# Used for local builds. This script is not used in the CI/CD pipeline.

targets=(
  "aarch64-apple-darwin"
  "x86_64-pc-windows-gnu"
  "x86_64-unknown-linux-gnu"
)

for target in "${targets[@]}"
do
  echo "Building for target: $target"
  cargo build --target "$target" --release
  if [ $? -ne 0 ]; then
    echo "Build failed for target: $target"
    exit 1
  fi
done

echo "All builds completed successfully."
