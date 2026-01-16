#!/bin/bash
cd "$(dirname "$0")"

if [ -f "comfy-qa-linux" ]; then
    chmod +x comfy-qa-linux
    ./comfy-qa-linux
elif [ -f "cli/target/release/comfy-qa" ]; then
    ./cli/target/release/comfy-qa
else
    echo "ERROR: Cannot find comfy-qa"
    read -p "Press Enter to exit..."
fi
