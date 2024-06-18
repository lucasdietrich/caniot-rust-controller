.PHONY: build run target deploy deploy_release deploy_config deploy_static fmt ui

all: build

format:
	cargo fmt

build: ui
	cargo build --features "emu"

clippy:
	cargo clippy -- -D warnings

run:
	cargo run --features "emu" --profile dev

clean:
	cargo clean

target:
	./scripts/build.sh build debug

deploy: deploy_config deploy_static
	./scripts/build.sh build debug
	scp target/armv7-unknown-linux-gnueabihf/debug/caniot-controller rpi:/home/root/rust-controller/caniot-controller

deploy_static:
	ssh rpi "mkdir -p /home/root/rust-controller/ui/dist"
	scp -rp ui/dist/* rpi:/home/root/rust-controller/ui/dist

deploy_release: deploy_config deploy_static
	./scripts/build.sh build release
	scp target/armv7-unknown-linux-gnueabihf/release/caniot-controller rpi:/home/root/rust-controller/caniot-controller

deploy_config:
	ssh rpi "mkdir -p /home/root/rust-controller"
	scp scripts/caniot-controller.toml rpi:/home/root/rust-controller/caniot-controller.toml

ui:
	make -C proto/grpc-web
	make -C ui

ui_clean:
	make -C proto/grpc-web clean
	make -C ui clean

ui_run:
	make -C ui run

ui_rebuild: ui_clean ui

bitbake:
	cargo-bitbake bitbake -R