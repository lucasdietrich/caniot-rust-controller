.PHONY: build run target deploy deploy_release deploy_config fmt

format:
	cargo fmt

build:
	cargo build

clean:
	cargo clean

run:
	cargo run

target:
	./scripts/build.sh build debug

deploy: deploy_config
	./scripts/build.sh build debug
	scp target/armv7-unknown-linux-gnueabihf/debug/caniot-rctrl rpi:/home/root/caniot-rctrl

deploy_release: target
	./scripts/build.sh build release
	scp target/armv7-unknown-linux-gnueabihf/release/caniot-rctrl rpi:/home/root/caniot-rctrl

deploy_config:
	scp scripts/caniot-controller.toml rpi:/home/root/caniot-controller.toml

fmt:
	cargo fmt && cargo clippy -- -D warnings