install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component
	gem install wasmtime

clean:
	cargo clean
	rm src/bindings.rs

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasip1/release/*.wasm run/

build: component
	cp tests/ruby.rb run/ruby.rb

test:
	ruby run/ruby.rb

check:
	wasm-tools validate run/wasm_demo.wasm
	wasm-tools component wit run/wasm_demo.wasm
	wasm-tools print run/wasm_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	gem list | grep wasmtime

lint:
	rustfmt src/lib.rs
