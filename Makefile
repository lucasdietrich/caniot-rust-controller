.PHONY: build run target deploy deploy_release deploy_config deploy_static fmt ui

all: build

format:
	cargo fmt

build: ui
	cargo build

test:
	cargo test

clippy:
	cargo clippy -- -D warnings

run:
	cargo run --profile dev --bin caniot-controller

clean:
	cargo clean

target:
	./scripts/build.sh build debug

target_release:
	./scripts/build.sh build release

deploy_release: deploy_config deploy_static deploy_bin_release
deploy_debug: deploy_config deploy_static deploy_bin_debug

deploy_static:
	ssh $(TARGET) "mkdir -p /home/root/rust-controller/ui/dist"
	scp -rp ui/dist/* $(TARGET):/home/root/rust-controller/ui/dist

deploy_config:
	scp scripts/caniot-controller.toml $(TARGET):~

TARGET_ARCH ?= armv7-unknown-linux-gnueabihf
TARGET ?= rpi3dev
BUILDDIR := $(or $(CARGO_RUST_TARGET),target)

deploy_bin_release: target_release
	scp $(BUILDDIR)/$(TARGET_ARCH)/release/caniot-controller $(TARGET):~

deploy_bin_debug: target
	scp $(BUILDDIR)/$(TARGET_ARCH)/debug/caniot-controller $(TARGET):~

echo:
	echo "TARGET_ARCH: $(TARGET_ARCH)"

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

kill:
	./scripts/kill-caniot-controller.sh