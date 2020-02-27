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
docker inspect hermod-server | rg IPAddress

sleep 2

# Start client
echo "Starting client"
docker run -it --rm --name hermod-client -v sources:/sources hermod-client
docker inspect hermod-client | rg IPAddress

# Compare hashes
echo "Comparing hashes..."

for FILE in "${srcs[@]}"; do
    printf "Checking $FILE: "

    expected=$(b3sum $SRC_DIR/$FILE)
    received=$(b3sum $OUT_DIR/$FILE)

    if [ "$expected" = "$received" ]; then
        printf "Ok {$expected}\n"
    else
        printf "Failed\n"
    fi

done

docker stop hermod-server &>/dev/null &

rm -r sources
rm -r output
