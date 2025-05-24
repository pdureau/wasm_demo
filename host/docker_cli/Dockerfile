FROM scratch
COPY --chmod=755 tmp/wasi_cli_rust.wasm /wasi_cli_rust.wasm
ENTRYPOINT ["/wasi_cli_rust.wasm"]
