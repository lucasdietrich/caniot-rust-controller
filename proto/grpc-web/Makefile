.PHONY: build dependencies clean

all: build

dependencies:
	yarn install

build: dependencies
	./proto-gen-grpc-web.sh

clean:
	rm -rf node_modules
	rm api/*