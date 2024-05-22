import wasmtime, wasmtime.loader
import wasm_meetup_demo

store = wasmtime.Store()
component = wasm_meetup_demo.Root(store)
print(component.add(store, 2, 3))
