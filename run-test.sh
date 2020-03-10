#!/bin/bash

mkdir sources
mkdir output

SRC_DIR="/sources"
OUT_DIR="/output"

REMOTE="test"
HOST_NAME=hermod-server

srcs=(large.file medium.file small.file)

set -e

# Share key with server
hermod share-key --host $HOST_NAME --name $REMOTE

# Upload 3 files
echo "Uploading $srcs[@]"
hermod upload --source $SRC_DIR/ --remote $REMOTE --destination $OUT_DIR

echo "Downloading ${srcs[@]}"
hermod download --source $OUT_DIR/ --remote $REMOTE --destination $OUT_DIR

for FILE in "${srcs[@]}"; do
    printf "Checking $FILE: "

    expected=$(b3sum --no-names $SRC_DIR/$FILE)
    received=$(b3sum --no-names $OUT_DIR/$FILE)

    if [ "$expected" = "$received" ]; then
        printf "Ok {$expected}\n"
    else
        printf "Failed\n"
    fi

done
