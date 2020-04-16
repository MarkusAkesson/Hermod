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

srcs=(large.file medium.file small.file)


# Share keys with server
hermod share-key --host $HOST_NAME --name $REMOTE
ssh-keygen -t rsa -b 4096 -f ~/.ssh/id_rsa
ssh-copy-id -i ~/.ssh/id_rsa root@$HOST_NAME

# Upload 3 files
for FILE in "${srcs[@]}"; do
    echo "Uploading $FILE using hermod"
    hyperfine "hermod upload --source $SRC_DIR/$FILE --remote $REMOTE --destination $OUT_DIR" #--show-output
done

for FILE in "${srcs[@]}"; do
    echo "Uploading $FILE using scp"
    hyperfine "scp $SRC_DIR/$FILE root@$HOST_NAME:$OUT_DIR" #--show-output
done

for FILE in "${srcs[@]}"; do
    echo "Uploading $FILE using sftp"
    hyperfine "sftp root@$HOST_NAME <<EOF
        put $SRC_DIR/$FILE $OUT_DIR
        EOF" #--show-output
done
