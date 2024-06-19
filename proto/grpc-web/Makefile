.PHONY: build dependencies clean

all: build

dependencies:
	npm install

build: dependencies
	./proto-gen-grpc-web.sh

clean:
	rm -rf node_modules
	rm -rf api/*