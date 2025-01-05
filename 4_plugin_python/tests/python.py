import wasmtime, wasmtime.loader
import wasm_demo

store = wasmtime.Store()
component = wasm_demo.Root(store)
print(component.greet(store, "Hello", true))
