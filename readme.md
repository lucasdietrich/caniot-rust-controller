# CANIOT Rust Controller

## Build for Raspberry Pi 3

Requirements:

- [Rust](https://www.rust-lang.org/tools/install) version 1.71.0
  - Rustup target `armv7-unknown-linux-gnueabihf` (install with `rustup target add armv7-unknown-linux-gnueabihf`)
- Yocto Poky SDK for Raspberry Pi 3 (_Hypirl_ personnal distribution)
- `sudo dnf install protobuf-compiler protobuf-devel`
- `sudo dnf install grpcurl` (optional: for testing purpose)

Configure `~/.cargo/config` with

```
[target.armv7-unknown-linux-gnueabihf]
linker = "/opt/poky/hypirl-rpi-1.0/sysroots/x86_64-pokysdk-linux/usr/bin/arm-poky-linux-gnueabi/arm-poky-linux-gnueabi-gcc"
rustflags = [
"-C", "link-arg=-mthumb",
"-C", "link-arg=-mfpu=neon-vfpv4",
"-C", "link-arg=-mfloat-abi=hard",
"-C", "link-arg=-mcpu=cortex-a7",
"-C", "link-arg=-fstack-protector-strong",
"-C", "link-arg=-O2",
"-C", "link-arg=-D_FORTIFY_SOURCE=2",
"-C", "link-arg=-Wformat",
"-C", "link-arg=-Wformat-security",
"-C", "link-arg=--sysroot=/opt/poky/hypirl-rpi-1.0/sysroots/cortexa7t2hf-neon-vfpv4-poky-linux-gnueabi",
]
```

Build with

    cargo build --target=armv7-unknown-linux-gnueabihf --release --verbose`

## Run

Configuration file is `caniot-controller.toml` in the current directory.

```
[can]
interface = "can0"

[web]
port = 8080
listen = "0.0.0.0"

[grpc]
listen = "[::]:50051"
```

Run with

    ./caniot-rust-controller

## TODO (controller)

- Allow to set custom configuration file path: `./caniot-rust-controller -c /etc/caniot/caniot-controller.toml`
- Timestamp received CAN frames (wait for this feature of `socketcan` crate https://github.com/socketcan-rs/socketcan-rs/issues/22)
- Improve logging: make this shorter `[2023-09-24T09:46:24.759Z INFO  caniot_rctrl::can] RX Telemetry Response (9: 1,1): ep-2 / 12 00 11 00 14 00 22 00` -> `[2023-09-24T09:46:24.759Z INFO can] RX Telemetry Response (9: 1,1): ep-2 / 12 00 11 00 14 00 22 00`
- Logger
- Database

## TODO (UI)

- Solve the resolution problem of: `import { Timestamp } from "google-protobuf/google/protobuf/timestamp_pb";`
