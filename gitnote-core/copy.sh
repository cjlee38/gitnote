#!/bin/bash

# Declare an array for targets and binaries
targets=(
  "aarch64-apple-darwin git-note"
  "x86_64-pc-windows-gnu git-note.exe"
  "x86_64-unknown-linux-gnu git-note"
)

# Iterate through the array
for target_info in "${targets[@]}"
do
  # Split the target_info string into target and binary
  IFS=' ' read -r target binary <<< "$target_info"

  echo "Copy for target: $target"
  dest="../gitnote-jetbrains/src/main/resources/core/$target/"
  mkdir -p "$dest"
  cp "target/$target/release/$binary" "$dest"
  if [ $? -ne 0 ]; then
    echo "Copy failed for target: $target"
    exit 1
  fi
done

echo "All copy completed successfully."
