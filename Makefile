format:
	cargo fmt

build:
	cargo build

run:
	cargo run

target_build:
	./scripts/build.sh build

target_run: 
	ssh rpi /home/root/caniot-rctrl

deploy: target_build
	scp target/armv7-unknown-linux-gnueabihf/debug/caniot-rctrl rpi:/home/root/caniot-rctrl

deploy_release: target_build
	scp target/armv7-unknown-linux-gnueabihf/release/caniot-rctrl rpi:/home/root/caniot-rctrl

deploy_config:
	scp scripts/caniot-controller.toml rpi:/home/root/caniot-controller.toml

fmt:
	cargo fmt && cargo clippy -- -D warnings