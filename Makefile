.PHONY: build run target deploy deploy_release deploy_config deploy_static fmt

all: build

format:
	cargo fmt

build:
	cargo build --features "emu"

run:
	cargo run --features "emu"

clean:
	cargo clean

target:
	./scripts/build.sh build debug

deploy: deploy_config deploy_static
	./scripts/build.sh build debug
	scp target/armv7-unknown-linux-gnueabihf/debug/caniot-rust-controller rpi:/home/root/caniot-rust-controller

deploy_release: deploy_config deploy_static
	./scripts/build.sh build release
	scp target/armv7-unknown-linux-gnueabihf/release/caniot-rust-controller rpi:/home/root/caniot-rust-controller

deploy_static:
	ssh rpi "mkdir -p /home/root/static"
	scp static/* rpi:/home/root/static
	ssh rpi "mkdir -p /home/root/templates/tera"
	scp templates/tera/* rpi:/home/root/templates/tera

deploy_config:
	scp scripts/caniot-controller.toml rpi:/home/root/caniot-controller.toml

clippy:
	cargo clippy -- -D warnings