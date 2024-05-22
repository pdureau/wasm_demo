install:
	cargo binstall cargo-component

clean:
	cargo clean
	rm src/bindings.rs

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasi/release/*.wasm run/

build: component

test:
	wasmtime run run/wasm_demo.wasm you false

check:
	wasm-tools validate run/wasm_demo.wasm
	wasm-tools component wit run/wasm_demo.wasm
	wasm-tools print run/wasm_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	wasmtime --version

lint:
	rustfmt src/main.rs
