#!/usr/bin/bash

ADDR='192.168.10.215:50051'
PROTO=./proto/model.proto

# parse first argument equals hello
if [ "$1" = "hello" ]; then
    grpcurl -plaintext -import-path ./proto -proto $PROTO \
        -d '{"name": "Lucas"}' \
        $ADDR \
        cancontroller.ipc.CanController/Hello
else
    echo "Unknown command"
    exit 1
fi
