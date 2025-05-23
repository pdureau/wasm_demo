FROM scratch
COPY --chmod=755 tmp/wasi_http_rust.wasm /wasi_http_rust.wasm
ENTRYPOINT ["/wasi_http_rust.wasm"]
