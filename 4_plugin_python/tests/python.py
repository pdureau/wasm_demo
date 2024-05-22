import wasmtime, wasmtime.loader
import wasm_demo

store = wasmtime.Store()
component = wasm_demo.Root(store)
print(component.add(store, 2, 3))
