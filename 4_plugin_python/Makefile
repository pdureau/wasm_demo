install:
	cargo binstall cargo-component
	pip install wasmtime

clean:
	cargo clean
	rm src/bindings.rs

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasi/release/*.wasm run/

build: component
	cp tests/python.py run/python.py

test:
	python run/python.py

test_2:
	python tests/python_2.py

check:
	wasm-tools validate run/wasm_meetup_demo.wasm
	wasm-tools component wit run/wasm_meetup_demo.wasm
	wasm-tools print run/wasm_meetup_demo.wasm  | head

versions:
	cargo -V
	cargo component --version
	npm list | grep jc

lint:
	rustfmt src/lib.rs
