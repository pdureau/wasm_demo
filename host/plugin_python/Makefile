install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	pip install wasmtime --upgrade

run:
	cp ../../hello_world_rust.wasm hello_world_rust.wasm
	python -m wasmtime.bindgen hello_world_rust.wasm --out-dir bindings
	python python.py

versions:
	pip show wasmtime | grep Version
