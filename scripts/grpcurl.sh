#!/usr/bin/bash

ADDR='localhost:50051'
PROTO=./proto/ng.proto

# parse first argument equals hello
if [ "$1" = "hello" ]; then
    grpcurl -plaintext -import-path ./proto -proto $PROTO \
        -d '{"name": "Lucas"}' \
        $ADDR \
        ng.InternalService/Hello
else
    echo "Unknown argument $1"
    exit 1
fi
