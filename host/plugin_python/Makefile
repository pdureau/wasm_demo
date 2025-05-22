install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	pip install wasmtime --upgrade

run:
	cp ../../hello_world_rust.wasm hello_world_rust.wasm
	python python.py

versions:
	pip show wasmtime | grep Version
