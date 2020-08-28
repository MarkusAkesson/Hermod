#!/bin/bash

set -e

if [ "$1" = "server" ]; then
    /usr/sbin/sshd
    hermod server
    exit
fi

HERMOD_SRC='/root/Hermod'
SRC_DIR="/sources"
OUT_DIR="/output"

REMOTE="bench"
HOST_NAME=eoan

srcs=large.file,medium.file,small.file,src

# Upload files
hyperfine --parameter-list src ${srcs[@]} "hermod upload --source $SRC_DIR/{src} --remote $REMOTE --destination $OUT_DIR" --export-json /output/hermod.json

hyperfine --parameter-list src ${srcs[@]} "scp -r $SRC_DIR/{src} root@$HOST_NAME:$OUT_DIR" --export-json /output/scp.json

hyperfine --parameter-list src ${srcs[@]} "sftp root@$HOST_NAME <<EOF
put -r $SRC_DIR/{src} $OUT_DIR
EOF" --export-json /output/sftp.json
