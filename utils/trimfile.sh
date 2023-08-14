#!/bin/bash

WORK_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
FILE="$WORK_DIR/../output.csv"
LINE_COUNT=$(wc -l < "$FILE")
LINE_LIMIT=10000

if [ "$LINE_COUNT" -gt $LINE_LIMIT ]; then
    # Trim file to retain only the last 10 million lines
    tail -n $LINE_LIMIT "$FILE" > "$FILE.tmp" && mv "$FILE.tmp" "$FILE"
fi
