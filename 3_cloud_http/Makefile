install:
	cargo binstall cargo-component

clean:
	cargo clean
	rm src/bindings.rs

build: component

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasip1/release/*.wasm run/

test: 
	wasmtime serve run/wasm_demo.wasm 

check:
	wasm-tools validate run/wasm_demo.wasm
	wasm-tools component wit run/wasm_demo.wasm
	wasm-tools print run/wasm_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	wasmtime --version

lint:
	rustfmt src/lib.rs
