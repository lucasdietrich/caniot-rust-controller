#!/bin/bash

NAME="caniot-rust-controller-full"
MODE="release"
TARGET="rpi"
POKY_ENV="/opt/poky/hypirl-rpi-1.0/environment-setup-cortexa7t2hf-neon-vfpv4-poky-linux-gnueabi"
TARGET_ARCH="armv7-unknown-linux-gnueabihf"

source $POKY_ENV

function build() {
    cargo build --target=$TARGET_ARCH --$MODE --verbose
}

function clean() {
    rm -rf target
}

function deploy() {
    scp target/$TARGET_ARCH/$MODE/$NAME $TARGET:/home/root
}

USAGE="Usage: $0 -d {build|clean|deploy}"

# Ensure that at least one argument was passed into the script
if [ $# -eq 0 ]; then
    echo "No arguments provided. Please provide a task."
    echo "$USAGE"
    exit 1
fi

# Determine which task should be executed
TASK="$1"

case $TASK in
    build)
        build
        ;;
    clean)
        clean
        ;;
    deploy)
        deploy
        ;;
    *)
        echo "Invalid task. Please provide a valid task."
        echo "$USAGE"
        exit 1
        ;;
esac

exit 0
