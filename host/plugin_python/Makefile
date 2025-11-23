install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	pip install wasmtime --upgrade

run:
	python hello_world_rust.py
	python hello_world_imports_rust.py
	python hello_world_js.py

versions:
	pip show wasmtime | grep Version
