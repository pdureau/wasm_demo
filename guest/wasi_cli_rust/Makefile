.PHONY: install clean build check versions tests lint

install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	rustup update
	rustup target add wasm32-wasip2
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component
	cargo install wit-deps-cli
	wit-deps
	cargo install wac-cli

clean:
	cargo clean
	rm src/bindings.rs

build:
	cargo component build --target wasm32-wasip2 --release
	cp target/wasm32-wasip2/release/wasi_cli_rust.wasm ../../
	wac plug ../../wasi_cli_rust.wasm --plug ../../hello_world_rust.wasm -o ../../wasi_cli_rust.wasm

check:
	wasm-tools validate ../../wasi_cli_rust.wasm
	wasm-tools component wit ../../wasi_cli_rust.wasm
	wasm-tools print ../../wasi_cli_rust.wasm | head

versions:
	rustc --version
	cargo -V
	cargo component --version
	wasmtime --version

test:
	wasmtime run ../../wasi_cli_rust.wasm you false

lint:
	rustfmt src/lib.rs
