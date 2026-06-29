#!/usr/bin/env bash
set -euo pipefail

# Slice screenshot.png into 3 equal horizontal strips and combine into a
# triptych banner for the README hero section.

INPUT="${1:-long-screenshot.png}"
OUTPUT="${2:-hero.png}"

HEIGHT=$(magick identify -format "%h" "$INPUT")
THIRD=$((HEIGHT / 3))
REST=$((THIRD + HEIGHT % 3))

magick "$INPUT" -crop "828x${THIRD}+0+0"     +repage /tmp/slice_top.png
magick "$INPUT" -crop "828x${THIRD}+0+${THIRD}" +repage /tmp/slice_mid.png
magick "$INPUT" -crop "828x${REST}+0+$((THIRD * 2))" +repage /tmp/slice_bot.png

magick /tmp/slice_top.png /tmp/slice_mid.png /tmp/slice_bot.png +append "$OUTPUT"

rm /tmp/slice_top.png /tmp/slice_mid.png /tmp/slice_bot.png

echo "Wrote $OUTPUT ($(magick identify -format '%wx%h' "$OUTPUT"))"
