# 2025-05-29 Status

All guests have the same phony targets:

- `make install`
- `make clean`
- `make build`
- `make check`
- `make versions`
- `make test`
- `make lint`

All hosts have the same phony targets:

- `make install`
- `make run`
- `make versions`

# Use case 1: wasip2 as an universal plugin format

## Guests

### hello_world_rust

Exports a custom interface:

```
export greeter;
```

✅ Compilation OK but we targeted `wasm32-unknown-unknown` instead of `wasm32-wasip2` because of [excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235).

## Hosts

### browsers_jco

With [bytecodealliance/jco](https://github.com/bytecodealliance/jco/)

⚠️ Transpilation to a module and bindings generation instead of "native" support.

Compatibility with guests:

- `hello_world_rust`: ✅ OK

### plugin_python

With [bytecodealliance/wasmtime-py](https://github.com/bytecodealliance/wasmtime-py/)

⚠️ Transpilation to a module and bindings generation instead of "native" support. Let's try to use `wasmtime.loader` instead.

Compatibility with guests:

- `hello_world_rust`: ✅ OK

### plugin_ruby

With [bytecodealliance/wasmtime-rb](https://github.com/bytecodealliance/wasmtime-rb/):

Compatibility with guests:

- `hello_world_rust`: ✅ OK

# Use case 2: wasip2 in cloud & edge computing

## Guests

### wasi_cli_rust (with wasi:cli/command)

Imports the custom interface:

```
include wasi:cli/command;
import greeter;
```

So we are using composing this component with `hello_world_rust` using [bytecodealliance/wac](https://github.com/bytecodealliance/wac).

✅ Compilation OK. Unfortunately, the composed component is heavier (35kb + 75kb = 110kb) than a component directly built with the two interfaces (75kb).

### wasi_http_rust (with wasi:http/incoming-handler)

No component composition here:

```
export wasi:http/incoming-handler;
export greeter;
```

✅ Compilation OK.

`wasi:http` 0.3 will be simplified.

### Test with both wasi:cli/command and wasi:http/incoming-handler

The component build OK but `wasmtime run ` doesn't import `wasi:http`:

```
wasmtime run test.wasm you false
Error: failed to run main module `test.wasm`

Caused by:
    0: component imports instance `wasi:http/types@0.2.5`, but a matching implementation was not found in the linker
    1: instance export `fields` has the wrong type
    2: resource implementation is missing
```

## Hosts

### docker_cli (with wasi:cli/command)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `wasi_cli_rust`: ✅
- `wasi_http_rust`: Not applicable

### docker_http (with wasi:http/incoming-handler)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `wasi_cli_rust`: ⚠️ Run without errors but not reachable from port 8080.
- `wasi_http_rust`: Not applicable
