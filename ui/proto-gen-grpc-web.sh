#!/bin/bash

mkdir -p src/api

rm -rf src/api/*.js
rm -rf src/api/*.ts

protoc -I=../proto ../proto/*.proto \
  --js_out=import_style=commonjs:./src/grpc-web-client-gen \
  --grpc-web_out=import_style=typescript,mode=grpcweb:./src/grpc-web-client-gen