install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	pip install wasmtime --upgrade

run:
	python -m wasmtime.bindgen ../../hello_world_rust.wasm --out-dir bindings/hello_world_rust
	python hello_world_rust.py
	python -m wasmtime.bindgen ../../hello_world_imports_rust.wasm --out-dir bindings/hello_world_imports_rust
	python hello_world_imports_rust.py

versions:
	pip show wasmtime | grep Version
