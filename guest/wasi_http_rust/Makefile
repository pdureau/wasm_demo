install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	rustup update
	# No wasip2 because of https://github.com/rust-lang/rust/issues/133235
	# rustup target add wasm32-wasip2
	rustup target add wasm32-unknown-unknown
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component
	cargo install wit-deps-cli
	wit-deps

clean:
	cargo clean
	rm src/bindings.rs

build:
	# No wasip2 because of https://github.com/rust-lang/rust/issues/133235
	# cargo component build --target wasm32-wasip2 --release
	# cp target/wasm32-wasip2/release/wasi_http_rust.wasm ../../
	cargo component build --target wasm32-unknown-unknown --release
	wasm-tools component wit target/wasm32-unknown-unknown/release/wasi_http_rust.wasm
	cp target/wasm32-unknown-unknown/release/wasi_http_rust.wasm ../../

check:
	wasm-tools validate ../../wasi_http_rust.wasm 
	wasm-tools component wit ../../wasi_http_rust.wasm 
	wasm-tools print ../../wasi_http_rust.wasm | head

versions:
	cargo -V
	cargo component --version
	wasmtime --version

test: 
	wasmtime serve -Scli ../../wasi_http_rust.wasm

lint:
	rustfmt src/lib.rs
