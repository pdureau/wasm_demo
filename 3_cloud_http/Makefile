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
	cp target/wasm32-wasi/release/*.wasm run/

test: 
	wasmtime serve run/wasm_meetup_demo.wasm 

check:
	wasm-tools validate run/wasm_meetup_demo.wasm
	wasm-tools component wit run/wasm_meetup_demo.wasm
	wasm-tools print run/wasm_meetup_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	wasmtime --version

lint:
	rustfmt src/lib.rs
