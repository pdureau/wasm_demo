.PHONY: install run versions

install:
	npm install @bytecodealliance/jco

run:
	npx jco transpile --no-typescript ../../hello_world_rust.wasm -o run/hello_world_rust/
	npx jco transpile --no-typescript ../../hello_world_imports_rust.wasm -o run/hello_world_imports_rust/
	rsync -a tests/ run/
	python3 -m http.server -d ./run

versions:
	npm list | grep jc

