.PHONY: install clean build check versions tests lint

install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	rustup update
	# No wasip2 because of https://github.com/rust-lang/rust/issues/133235
	# rustup target add wasm32-wasip2
	rustup target add wasm32-unknown-unknown
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component

clean:
	cargo clean
	rm src/bindings.rs

build:
	# No wasip2 because of https://github.com/rust-lang/rust/issues/133235
	# cargo component build --target wasm32-wasip2 --release
	# cp target/wasm32-wasip2/release/hello_world_rust.wasm ../../
	cargo component build --target wasm32-unknown-unknown --release
	wasm-tools component wit target/wasm32-unknown-unknown/release/hello_world_rust.wasm
	cp target/wasm32-unknown-unknown/release/hello_world_rust.wasm ../../

check:
	wasm-tools validate ../../hello_world_rust.wasm
	wasm-tools component wit ../../hello_world_rust.wasm
	wasm-tools print ../../hello_world_rust.wasm | head

versions:
	rustc --version
	cargo -V
	cargo component --version
	wasmtime --version

test:
	wasmtime run --invoke 'greet("Pierre", true)' ../../hello_world_rust.wasm

lint:
	rustfmt src/lib.rs
