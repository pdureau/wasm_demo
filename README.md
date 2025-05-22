# 2025-05-22 Status

## Guests

All guests have the same phony targets:

- `make install`
- `make clean`
- `make build`
- `make check`
- `make versions`
- `make test`
- `make lint`

### hello_world_rust

✅ Compilation OK but [requires excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235) is causing troubles with some hosts:

```
  import wasi:cli/environment@0.2.0;
  import wasi:cli/exit@0.2.0;
  import wasi:io/error@0.2.0;
  import wasi:io/streams@0.2.0;
  import wasi:cli/stdin@0.2.0;
  import wasi:cli/stdout@0.2.0;
  import wasi:cli/stderr@0.2.0;
  import wasi:clocks/wall-clock@0.2.0;
  import wasi:filesystem/types@0.2.0;
  import wasi:filesystem/preopens@0.2.0;
```

### wasi_cli_rust (with wasi:cli/run)

❌ Don't compile since we moved to wasip2.

```
error: linking with `wasm-component-ld` failed: exit status: 1
Caused by:
  0: failed to decode world from module
  1: module was not valid
  2: failed to find export of interface `pdureau:wasm-demo/greeter` function `greet`
```

Nice to have: Try composition to split the business logic from the CLI wrapper.

### wasi_http_rust (with wasi:http/proxy)

❌ Don't compile since we moved to wasip2.

```
error: linking with `wasm-component-ld` failed: exit status: 1
Caused by:
  0: failed to decode world from module
  1: module was not valid
  2: failed to find export of interface `pdureau:wasm-demo/greeter` function `greet`
```

Nice to have: Try composition to split the business logic from the HTTP wrapper.

## Hosts

All hosts have the same phony targets:

- `make install`
- `make run`
- `make versions`

### browsers_jco

With [bytecodealliance/jco](https://github.com/bytecodealliance/jco/)

OK but transpilation instead of "native" support.

Compatibility with guests:

- `hello_world_rust`: ✅ OK because of those imports:

```
"@bytecodealliance/preview2-shim/cli": "./packages/preview2-shim/lib/browser/cli.js",
"@bytecodealliance/preview2-shim/filesystem": "./packages/preview2-shim/lib/browser/filesystem.js",
"@bytecodealliance/preview2-shim/io": "./packages/preview2-shim/lib/browser/io.js",
"@bytecodealliance/preview2-shim/sockets": "./packages/preview2-shim/lib/browser/sockets.js"
```

That's weird because there are not exactly the same than the ones added to the guest: sockets has replaced clocks.

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable

### cloud_cli (with wasi:cli/run)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `hello_world_rust`: Not applicable
- `wasi_cli_rust`: 🕑 Waiting the guest to be ready
- `wasi_http_rust`: Not applicable

### cloud_http (with wasi:http/proxy)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `hello_world_rust`: Not applicable
- `wasi_cli_rust`: 🕑 Waiting the guest to be ready
- `wasi_http_rust`: Not applicable

### plugin_python

With [bytecodealliance/wasmtime-py](https://github.com/bytecodealliance/wasmtime-py/)

Compatibility with guests:

- `hello_world_rust`: ❌ Doesn't work because of unexpected the extra imports: `not implemented: imported resources not yet supported`
- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable

### plugin_ruby

with [bytecodealliance/wasmtime-rb](https://github.com/bytecodealliance/wasmtime-rb/):

Compatibility with guests:

- `hello_world_rust`: ❌ Doesn't work because of unexpected the extra imports:

```
component imports instance `wasi:cli/environment@0.2.0`, but a matching implementation was not found in the linker
```

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable
