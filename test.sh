#!/bin/bash

mkdir sources
mkdir output

SRC_DIR="$(pwd)/sources"
OUT_DIR="$(pwd)/output/"
DEST_DIR="/tmp/"
REMOTE="test"
HOST_IP=127.0.0.1

fallocate -l 1G $SRC_DIR/large.file
fallocate -l 500M $SRC_DIR/medium.file
fallocate -l 10K $SRC_DIR/small.file

srcs=(large.file medium.file small.file)

# Build and start server
docker build -t hermod .
docker run -dit --rm --name hermod-server --network=host hermod

# Share key with server
cargo run -- share-key --host $HOST_IP --name $REMOTE

# Upload 3 files
pids=()
for FILE in "${srcs[@]}"; do
    echo "Transfering $FILE"
    cargo run -- upload --source $SRC_DIR/$FILE --remote $REMOTE --destination $DEST_DIR &
    pids+=($!)
done

# Await processes to finish
for pid in ${pids[*]}; do
    wait $pid
done

# Retrive files
for FILE in "${srcs[@]}"; do
    echo "Retrieving $FILE"
    docker cp hermod-server:$DEST_DIR/$FILE $OUT_DIR/$FILE
done

# Compare hashes
echo "Comparing hashes..."

for FILE in "${srcs[@]}"; do
    printf "Checking $FILE"

    expected=(b3sum $SRC_DIR/$FILE)
    received=(b3sum $OUT_DIR/$FILE)

    if [ "$expected" = "$received" ]; then
        printf ": Ok\n"
    else
        printf ": Failed\n"
    fi

done

docker stop hermod-server &>/dev/null &

rm -r sources
rm -r output
