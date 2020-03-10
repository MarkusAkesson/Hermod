#!/bin/bash

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

docker network create test

# Start server
echo "Starting server"
docker run -dit --rm --name hermod-server --network test hermod-server

sleep 2

# Start client
echo "Starting client"
docker run -it --name hermod-client --network test hermod-client

# Cleaning up
echo "Cleaning up"
docker stop hermod-server &>/dev/null
docker rm hermod-client &>/dev/null
docker network rm test &>/dev/null
