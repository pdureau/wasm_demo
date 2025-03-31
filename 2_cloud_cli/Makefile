install:
	curl https://wasmtime.dev/install.sh -sSf | bash
	cargo install cargo-binstall
	cargo binstall wasm-tools
	cargo binstall cargo-component

install_docker_shim:
	git clone git@github.com:containerd/runwasi.git
	cd runwasi/
	sudo dnf install libseccomp-devel
	sudo dnf install protobuf-compiler
	make build-wasmtime
	make install-wasmtime

install_docker: install_docker_shim

clean:
	cargo clean
	rm src/bindings.rs

component:
	# rm -rf run/*
	cargo component build --release
	mkdir -p run/
	cp target/wasm32-wasip1/release/*.wasm run/

build: component

build_docker: component
	docker build --load -t wasm_demo .

test:
	wasmtime run run/wasm_demo.wasm you false

test_docker:
	docker run --rm --runtime=io.containerd.wasmtime.v1 wasm_demo Belgium true

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
