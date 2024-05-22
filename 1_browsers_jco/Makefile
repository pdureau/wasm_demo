install:
	cargo binstall cargo-component
	npm install @bytecodealliance/jco

clean:
	cargo clean
	rm src/bindings.rs

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasi/release/*.wasm run/

build: component
	npx jco transpile --no-typescript run/wasm_demo.wasm -o run/jco/
	mkdir -p run/jco/packages/preview2-shim
	cp -rf node_modules/@bytecodealliance/preview2-shim run/jco/packages/
	cp tests/jco.html run/jco/test.html

test:
	python3 -m http.server

check:
	wasm-tools validate run/wasm_demo.wasm
	wasm-tools component wit run/wasm_demo.wasm
	wasm-tools print run/wasm_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	npm list | grep jc

lint:
	rustfmt src/lib.rs
