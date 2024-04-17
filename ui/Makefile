.PHONY: build server dependencies clean

build: dependencies
	yarn build

dependencies:
	yarn install
	yarn add file:../proto/grpc-web

clean:
	rm -rf build

format:
	./node_modules/.bin/prettier --write .
