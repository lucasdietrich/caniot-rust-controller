#!/usr/bin/bash

pid=$(ps -af | grep target/debug/caniot-controller | grep -v grep | awk '{print $2}')

if [ -z "$pid" ]; then
    echo "caniot-controller is not running"
else
    echo "killing caniot-controller with pid $pid"
    kill -9 $pid
fi