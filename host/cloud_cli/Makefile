.PHONY: install run

install:
	sudo dnf install docker
	sudo systemctl start docker
	- mkdir tmp/
	wget -qO- https://github.com/containerd/runwasi/releases/download/containerd-shim-wasmtime%2Fv0.6.0/containerd-shim-wasmtime-x86_64-linux-musl.tar.gz | gunzip | tar xvf - -C tmp/
	chmod +x tmp/containerd-shim-wasmtime-v1
	sudo install tmp/containerd-shim-wasmtime-v1 /usr/local/bin/

run:
	cp ../../wasi_cli_rust.wasm tmp/wasi_cli_rust.wasm
	sudo docker build  --no-cache --load -t wasm_demo_cli .
	sudo docker container run --rm --runtime=io.containerd.wasmtime.v1 wasm_demo_cli Belgium true

