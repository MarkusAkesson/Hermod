#!/bin/bash

set -e

if [ "$1" = "server" ]; then
    /usr/sbin/sshd
    hermod server --no-daemon
    exit
fi

SRC_DIR="/sources"
OUT_DIR="/output"

REMOTE="bench"
HOST_NAME=bench-server

srcs=large.file,medium.file,small.file,src


# Share keys with server
hermod share-key --host $HOST_NAME --name $REMOTE
ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa
ssh-copy-id -i ~/.ssh/id_rsa root@$HOST_NAME

# Upload files
hyperfine --parameter-list src ${srcs[@]} "hermod upload --source $SRC_DIR/{src} --remote $REMOTE --destination $OUT_DIR" --export-json /output/hermod.json

hyperfine --parameter-list src ${srcs[@]} "scp -r $SRC_DIR/{src} root@$HOST_NAME:$OUT_DIR" --export-json /output/scp.json

hyperfine --parameter-list src ${srcs[@]} "sftp root@$HOST_NAME <<EOF
put -r $SRC_DIR/{src} $OUT_DIR
EOF" --export-json /output/sftp.json
