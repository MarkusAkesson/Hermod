#!/bin/bash

mkdir sources
mkdir output

SRC_DIR="sources"
OUT_DIR="output"
REMOTE="test"

SERVER_IP=172.17.0.2
CLIENT_IP=172.17.0.3

srcs=(large.file medium.file small.file)

# Exit on first error
set -e

# Build server and client

if [ "$1" = "build" ]; then
    echo "Building containers"
    docker build -f Server.Dockerfile -t hermod-server .
    docker build -f Client.Dockerfile -t hermod-client .
else
    echo "Reusing old containers"
fi

# Start server
echo "Starting server"
docker run -dit --rm --name hermod-server -v output:/output hermod-server

sleep 2

# Start client
echo "Starting client"
docker run -it --name hermod-client -v sources:/sources hermod-client

for FILE in "${srcs[@]}"; do
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

docker stop hermod-server &>/dev/null &
docker rm hermod-client

rm -r sources
rm -r output
