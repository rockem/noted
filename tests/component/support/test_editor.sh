#!/bin/sh
if [ -f "$1" ]; then
    cat "$1"
else
    echo "new note" > "$1"
fi
