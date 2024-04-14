#!/bin/bash

rm -rf api
rm -rf api2

mkdir -p api
mkdir -p api2

# protoc -I=.. ../*.proto \
#   --plugin=protoc-gen-grpc-web=./node_modules/.bin/protoc-gen-grpc-web \
#   --js_out=import_style=commonjs,binary:./api \
#   --grpc-web_out=import_style=commonjs+dts,mode=grpcweb:./api

protoc -I=.. ../*.proto \
  --plugin=protoc-gen-grpc-web=./node_modules/.bin/protoc-gen-grpc-web \
  --js_out=import_style=commonjs:./api2 \
  --grpc-web_out=import_style=typescript,mode=grpcweb:./api2

# protoc -I=.. ../*.proto \
#   --plugin=protoc-gen-grpc-web=./node_modules/.bin/protoc-gen-grpc-web \
#   --js_out=import_style=commonjs:./api \
#   --grpc-web_out=import_style=commonjs,mode=grpcwebtext:./api

protoc -I=.. ../*.proto \
  --plugin=protoc-gen-ts=./node_modules/ts-protoc-gen/bin/protoc-gen-ts \
  --ts_out=./api