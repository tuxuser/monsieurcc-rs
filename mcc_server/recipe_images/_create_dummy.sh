#!/bin/sh

echo "[+] Creating dummy images"

convert -size 254x373 canvas:grey small-dummy.jpg
convert -size 750x1000 canvas:grey dummy.jpg
convert -size 1024x426 canvas:grey large-dummy.jpg