#!/bin/bash

NAME="caniot-controller"
TARGET="rpi"
TARGET_ARCH="${TARGET_ARCH:-armv7-unknown-linux-gnueabihf}"

# parse second argument as build type
if [ "$2" == "release" ]; then
    BUILD_TYPE="release"
else
    BUILD_TYPE="debug"
fi

# check for --verbose flag in any argument
VERBOSE=""
for arg in "$@"; do
    if [ "$arg" == "--verbose" ] || [ "$arg" == "-v" ]; then
        VERBOSE="--verbose"
    fi
done

function build() {
    # if release add --release
    if [ "$BUILD_TYPE" == "release" ]; then
        cargo build --target=$TARGET_ARCH --release $VERBOSE $FEATURES
    else
        cargo build --target=$TARGET_ARCH $VERBOSE $FEATURES
    fi
}

function clean() {
    rm -rf target
}

function deploy() {
    scp target/$TARGET_ARCH/$BUILD_TYPE/$NAME $TARGET:/home/root
}

USAGE="Usage: $0 {build|clean|deploy} [release] [-v|--verbose]"

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
