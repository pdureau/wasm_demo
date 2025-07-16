.PHONY: install clean build check versions tests lint

install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	npm install -g @bytecodealliance/componentize-js

clean:
	echo

build:
	componentize-js -d stdio -d random -d clocks -d http --wit wit -o ../../hello_world_js.wasm hello.js

check:
	wasm-tools validate ../../hello_world_js.wasm
	wasm-tools component wit ../../hello_world_js.wasm
	wasm-tools print ../../hello_world_js.wasm | head
	wasm-tools metadata show ../../hello_world_js.wasm

versions:
	wasmtime --version

test:
	# wasmtime run --invoke 'greet("Pierre", true)' ../../hello_world_rust.wasm
	echo "Not possible to run 'wasmtime run --invoke' when component has imports."

lint:
	prettier --write hello.js
