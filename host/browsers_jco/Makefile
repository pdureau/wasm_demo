.PHONY: install run versions

install:
	npm install @bytecodealliance/jco

run:
	npx jco transpile --no-typescript ../../hello_world_rust.wasm -o run/
	mkdir -p run/packages/preview2-shim
	cp -rf node_modules/@bytecodealliance/preview2-shim run/packages/
	cp tests/jco.html run/index.html
	python3 -m http.server

versions:
	npm list | grep jc

