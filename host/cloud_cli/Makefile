install:
	sudo dnf install docker
	sudo systemctl start docker
	- mkdir tmp/
	wget -qO- https://github.com/containerd/runwasi/releases/download/containerd-shim-wasmtime%2Fv0.6.0/containerd-shim-wasmtime-x86_64-linux-musl.tar.gz | gunzip | tar xvf - -C tmp/
	chmod +x tmp/containerd-shim-wasmtime-v1
	sudo install tmp/containerd-shim-wasmtime-v1 /usr/local/bin/

run:
	cp ../../hello_world_rust.wasm tmp/hello_world_rust.wasm
	sudo docker build --load -t wasm_demo .
	sudo docker run --rm --runtime=io.containerd.wasmtime.v1 wasm_demo Belgium true

