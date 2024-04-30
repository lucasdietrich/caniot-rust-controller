.PHONY: build server dependencies clean

all: build

dependencies:
	yarn install

build: dependencies
	yarn build

clean:
	rm -rf node_modules

run:
	npm run dev

format:
	./node_modules/.bin/prettier --write .
