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

Exports a custom interface:

```
export greeter;
```

⚠️ Compilation OK but [excessive WASI interfaces](https://github.com/rust-lang/rust/issues/133235) are causing troubles with some hosts:

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

### wasi_cli_rust (with wasi:cli/command)

Imports the custom interface:

```
include wasi:cli/command;
import greeter;
```

So we are using composing this component with `hello_world_rust` using [bytecodealliance/wac](https://github.com/bytecodealliance/wac).

✅ Compilation OK. Unfortunately, the composed component is twice heavier (150kb) than a component directly built with the two interfaces (75kb).

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

All hosts have the same phony targets:

- `make install`
- `make run`
- `make versions`

### browsers_jco

With [bytecodealliance/jco](https://github.com/bytecodealliance/jco/)

OK but transpilation instead of "native" support.

Compatibility with guests:

- `hello_world_rust`: ✅ OK because of those shims:

```
"@bytecodealliance/preview2-shim/cli": "./packages/preview2-shim/lib/browser/cli.js",
"@bytecodealliance/preview2-shim/filesystem": "./packages/preview2-shim/lib/browser/filesystem.js",
"@bytecodealliance/preview2-shim/io": "./packages/preview2-shim/lib/browser/io.js",
"@bytecodealliance/preview2-shim/sockets": "./packages/preview2-shim/lib/browser/sockets.js"
```

That's weird because there are not exactly the same than the ones added to the guest: sockets has replaced clocks.

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable

### cloud_cli (with wasi:cli/command)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `hello_world_rust`: Not applicable
- `wasi_cli_rust`: ✅
- `wasi_http_rust`: Not applicable

### cloud_http (with wasi:http/incoming-handler)

Based on [containerd/runwasi](https://github.com/containerd/runwasi).

Compatibility with guests:

- `hello_world_rust`: Not applicable
- `wasi_cli_rust`: ⚠️ Run without errors but not reachable from port 8080.
- `wasi_http_rust`: Not applicable

### plugin_python

With [bytecodealliance/wasmtime-py](https://github.com/bytecodealliance/wasmtime-py/)

Compatibility with guests:

- `hello_world_rust`: ❌ Doesn't work because of unexpected the extra imports:

```
not implemented: imported resources not yet supported
```

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable

### plugin_ruby

With [bytecodealliance/wasmtime-rb](https://github.com/bytecodealliance/wasmtime-rb/):

Compatibility with guests:

- `hello_world_rust`: ❌ Doesn't work because of unexpected the extra imports:

```
component imports instance `wasi:cli/environment@0.2.0`, but a matching implementation was not found in the linker
```

- `wasi_cli_rust`: Not applicable
- `wasi_http_rust`: Not applicable
