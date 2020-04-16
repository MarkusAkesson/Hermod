#!/bin/bash

# Exit on first error
trap cleanup EXIT

# Build server and client

if [ "$1" = "build" ]; then
    echo "Building containers"
    docker build -f BenchServer.Dockerfile -t bench-server .
    docker build -f BenchClient.Dockerfile -t bench-client .
else
    echo "Reusing old containers"
fi

docker network create bench

# Start server
echo "Starting server"
docker run -dit --rm --name bench-server --network bench bench-server

sleep 2

# Start client
echo "Starting client"
docker run -it --rm --name bench-client --network bench bench-client

function cleanup() {
    # Cleaning up
    echo "Cleaning up"
    docker stop bench-server &>/dev/null
    docker network rm bench &>/dev/null
}
