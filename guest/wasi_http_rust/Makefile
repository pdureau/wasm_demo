install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	rustup update
	rustup target add wasm32-wasip2
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component
	cargo install wit-deps-cli
	wit-deps

clean:
	cargo clean
	rm src/bindings.rs

build:
	cargo component build --target wasm32-wasip2 --release
	cp target/wasm32-wasip2/release/*.wasm ../../

check:
	wasm-tools validate ../../wasi_http_rust.wasm 
	wasm-tools component wit ../../wasi_http_rust.wasm 
	wasm-tools print ../../wasi_http_rust.wasm m  | head

versions:
	cargo -V
	cargo component --version
	wasmtime --version

test: 
	wasmtime serve ../../wasi_http_rust.wasm

lint:
	rustfmt src/lib.rs
