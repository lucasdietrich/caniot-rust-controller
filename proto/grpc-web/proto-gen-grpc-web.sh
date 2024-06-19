#!/bin/bash

mkdir -p api

rm -rf api/*.js
rm -rf api/*.ts

export PATH="$PATH:./node_modules/.bin"

# https://github.com/grpc/grpc-web/blob/master/README.md#client-configuration-options
protoc -I=.. \
    ../common.proto \
    ../legacy.proto \
    ../ng_alarms.proto \
    ../ng_controller.proto \
    ../ng_devices.proto \
    ../ng_garage.proto \
    ../ng_heaters.proto \
    ../ng_internal.proto \
  --js_out=import_style=commonjs:./api \
  --grpc-web_out=import_style=typescript,mode=grpcweb:./api

# Willingly excluding ng_can_iface.proto as it doesn't need to be generated for the web client