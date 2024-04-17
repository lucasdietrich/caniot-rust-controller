#!/bin/bash

mkdir -p api

rm -rf api/*.js
rm -rf api/*.ts

# https://github.com/grpc/grpc-web/blob/master/README.md#client-configuration-options
protoc -I=.. ../*.proto \
  --js_out=import_style=commonjs:./api \
  --grpc-web_out=import_style=typescript,mode=grpcweb:./api