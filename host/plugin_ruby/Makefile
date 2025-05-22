install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	gem install wasmtime

run:
	cp ../../hello_world_rust.wasm hello_world_rust.wasm
	ruby ruby.rb

versions:
	wasmtime --version
	gem list | grep wasmtime
