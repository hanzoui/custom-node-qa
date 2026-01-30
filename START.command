#!/bin/bash
cd "$(dirname "$0")"

if [ -f "comfy-qa-mac" ]; then
    chmod +x comfy-qa-mac
    ./comfy-qa-mac
elif [ -f "cli/target/release/comfy-qa" ]; then
    ./cli/target/release/comfy-qa
else
    echo "ERROR: Cannot find comfy-qa"
    read -p "Press Enter to exit..."
fi
