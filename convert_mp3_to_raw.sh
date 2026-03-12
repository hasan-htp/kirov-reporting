#!/bin/sh

RAW_DIR="raw_records"
mkdir -p "$RAW_DIR"

for file in records/*; do
    if [ -f "$file" ]; then
        echo "Processing $file"
        filename=$(basename "$file" .mp3)
        # sample rate is 48000
        # -ac (2 channel/stereo)
        # output format -f s16le (raw signed 16-bit little endian)
        ffmpeg -i "$file" -ar 48000 -ac 2 -f s16le "$RAW_DIR/$filename.raw"
    fi
done
