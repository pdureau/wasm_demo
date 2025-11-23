Demos of WebAssembly WASI Preview 2

# Use case 1: wasip2 as an universal plugin format

## Guests

### hello_world_rust

Exports a custom interface:

```
export greeter;
```

✅ Compilation OK but we targeted `wasm32-unknown-unknown` instead of `wasm32-wasip2` because of [excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235) and used

### hello_world_imports_rust

Exports and imports custom interfaces:

```
export greeter;
import formatter;
```

✅ Compilation OK but we targeted `wasm32-unknown-unknown` instead of `wasm32-wasip2` because of [excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235)

### hello_world_js

Exports and imports custom interfaces:

```
export greeter;
import formatter;
```

✅ "Compilation" OK with [bytecodealliance/ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS/). However, Wasm file weights 11MB (x333 the Rust example), including 8MB for the [StarlingMonkey](https://github.com/bytecodealliance/StarlingMonkey) engine.

## Hosts

### browsers_jco

With [bytecodealliance/jco](https://github.com/bytecodealliance/jco/)

⚠️ Transpilation to a module and bindings generation instead of "native" support.

Compatibility with guests:

- `hello_world_rust`: ✅ OK
- `hello_world_imports_rust`: ✅ OK
- `hello_world_js`: Not tested

### plugin_node

With [bytecodealliance/jco](https://github.com/bytecodealliance/jco/)

⚠️ Transpilation to a module and bindings generation instead of "native" support.

Compatibility with guests:

- `hello_world_rust`: ✅ OK
- `hello_world_imports_rust`: ✅ OK
- `hello_world_js`: No tested

### plugin_python

With [bytecodealliance/wasmtime-py](https://github.com/bytecodealliance/wasmtime-py/)

Compatibility with guests:

- `hello_world_rust`: ✅ OK
- `hello_world_imports_rust`: ✅ OK
- `hello_world_js`: ✅ OK

### plugin_ruby

With [bytecodealliance/wasmtime-rb](https://github.com/bytecodealliance/wasmtime-rb/):

Compatibility with guests:

- `hello_world_rust`: ✅ OK
- `hello_world_imports_rust`: ❌ [#433: Component host functions](https://github.com/bytecodealliance/wasmtime-rb/issues/433)
- `hello_world_js`: ❌ for the same reason.

# Use case 2: wasip2 in cloud & edge computing

## Guests

### wasi_cli_rust (with wasi:cli/run)

Imports the custom interface:

```
export wasi:cli/run;
import wasi:cli/environment;
import greeter;
```

So we are composing this component with `hello_world_rust` using [bytecodealliance/wac](https://github.com/bytecodealliance/wac).

✅ Compilation OK. Unfortunately, the composed component is heavier (35kb + 65kb = 100kb) than a component directly built with the two interfaces (75kb).

### wasi_http_rust (with wasi:http/incoming-handler)

No component composition here:

```
export wasi:http/incoming-handler;
export greeter;
```

✅ Compilation OK.

Note: `wasi:http` 0.3 will be simplified.

### Test with both wasi:cli/run and wasi:http/incoming-handler

The component is compiling without errors but `wasmtime run ` doesn't handle `wasi:http`:

```
wasmtime run test.wasm you false
Error: failed to run main module `test.wasm`

Caused by:
    0: component imports instance `wasi:http/types@0.2.5`, but a matching implementation was not found in the linker
    1: instance export `fields` has the wrong type
    2: resource implementation is missing
```

## Hosts

### docker_cli (with wasi:cli/run)

With [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `wasi_cli_rust`: ✅
- `wasi_http_rust`: Not applicable

### docker_http (with wasi:http/incoming-handler)

with [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: ⚠️ Run without errors but not reachable from port 8080.

### spin_http (with wasi:http/incoming-handler)

With Fermyon's Spin: https://spinframework.dev/

Compatibility with guests:

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: ✅

### wasmcloud_http (with wasi:http/incoming-handler)

With Wasmcloud: https://wasmcloud.com/

Compatibility with guests:

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: ❌

```
wasi:io/streams@0.2.5 instance import missing `input-stream` resource
```

# Makefiles

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
