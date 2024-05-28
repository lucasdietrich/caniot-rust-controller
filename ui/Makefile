.PHONY: build server dependencies clean

all: build

dependencies:
	npm install

build: dependencies
	npm run build

clean:
	rm -rf node_modules

run: dependencies
	npm run dev

format:
	./node_modules/.bin/prettier --write .
