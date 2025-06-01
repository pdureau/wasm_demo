.PHONY: install run versions

install:
	npm install @bytecodealliance/jco

run:
	npx jco transpile --no-typescript ../../hello_world_rust.wasm -o run/hello_world_rust/
	npx jco transpile --no-typescript ../../hello_world_imports_rust.wasm -o run/hello_world_imports_rust/ --map "pdureau:wasm-demo/formatter=./formatter.js" 
	rsync -a tests/ run/
	node run/hello_world_rust/index.js
	node run/hello_world_imports_rust/index.js

versions:
	npm list | grep jc

