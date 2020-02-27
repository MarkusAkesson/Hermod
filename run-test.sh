#!/bin/bash

mkdir sources
mkdir output

SRC_DIR="/sources"
DEST_DIR="/output"
REMOTE="test"
HOST_IP=172.17.0.2

srcs=(large.file medium.file small.file)

set -e

# Share key with server
hermod share-key --host $HOST_IP --name $REMOTE

# Upload 3 files
pids=()
for FILE in "${srcs[@]}"; do
    echo "Transfering $FILE"
    hermod upload --source $SRC_DIR/$FILE --remote $REMOTE --destination $DEST_DIR &
    pids+=($!)
done

# Await processes to finish
for pid in ${pids[*]}; do
    wait $pid
done

echo "Transfered ${srcs[@]}"