import wasmtime, wasmtime.loader
import addition

store = wasmtime.Store()
component = addition.Root(store)
print(component.add(store, 2, 3))
