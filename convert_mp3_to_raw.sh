#!/bin/sh

RAW_DIR="raw_records"
mkdir -p "$RAW_DIR"

for file in records/*; do
    if [ -f "$file" ]; then
        echo "Processing $file"
        filename=$(basename "$file" .mp3)
        ffmpeg -i "$file" -ar 16000 -f s16le "$RAW_DIR/$filename.raw"
    fi
done
