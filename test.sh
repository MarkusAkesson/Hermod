#!/bin/bash

mkdir sources
mkdir output

SRC_DIR="sources"
OUT_DIR="output"

srcs=(large.file medium.file small.file)

# Exit on first error
set -e

command -v b3sum >/dev/null 2>&1 || { echo >&2 "b3sum is required but it's not installed. Install it with 'cargo install b3sum'. Aborting."; exit 1; }


# Build server and client

if [ "$1" = "build" ]; then
    echo "Building containers"
    docker build -f Server.Dockerfile -t hermod-server .
    docker build -f Client.Dockerfile -t hermod-client .
else
    echo "Reusing old containers"
fi

docker network create test

# Start server
echo "Starting server"
docker run -dit --rm --name hermod-server -v output:/output --network test hermod-server

sleep 2

# Start client
echo "Starting client"
docker run -it --name hermod-client -v sources:/sources --network test hermod-client

# Fetching sent files to compare
for FILE in "${srcs[@]}"; do
    echo "Fetching $FILE"
    docker cp hermod-server:/$OUT_DIR/$FILE $OUT_DIR/
    docker cp hermod-client:/$SRC_DIR/$FILE $SRC_DIR/
done

# Compare hashes
echo "Comparing hashes..."

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

# Cleaning up
echo "Cleaning up"
docker stop hermod-server &>/dev/null
docker rm hermod-client &>/dev/null
docker network rm test &>/dev/null

rm -r sources
rm -r output
