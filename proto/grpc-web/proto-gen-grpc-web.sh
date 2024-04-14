#!/bin/bash

mkdir -p api

rm -rf api/*.js
rm -rf api/*.ts

protoc -I=.. ../*.proto \
  --js_out=import_style=commonjs:./api \
  --grpc-web_out=import_style=typescript,mode=grpcweb:./api