#!/bin/bash
#
# squeeze_images.sh
#
# Reduces image quality so as to reduce size.
#

FILES=images_original/*
OUT_DIR=static

for f in $FILES
do
    echo "Sqeezing $f file..."

    # Get the base name of the file
    base="$(basename -- $f)"

    # Read orientation from original image
    orientation=$(exiftool -Orientation -n -S $f | grep -Eo '[0-9]{1,4}')
    echo "Orientation $orientation"

    # Lower the imge quality
    ffmpeg -i $f -q:v 10 $OUT_DIR/$base

    # Restore orientation value for the new image
    exiftool -n -Orientation=$orientation $OUT_DIR/$base

done
