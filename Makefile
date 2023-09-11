format:
	cargo fmt

build:
	cargo build

run:
	cargo run

target_build:
	./scripts/build.sh build

deploy: target_build
	scp target/armv7-unknown-linux-gnueabihf/release/caniot-rust-controller-full rpi:/home/root/caniot-rust-controller-full

run: 
	ssh rpi /home/root/caniot-rust-controller-full

fmt:
	cargo fmt && cargo clippy -- -D warnings