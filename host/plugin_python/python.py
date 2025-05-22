import wasmtime, wasmtime.loader
import hello_world_rust

store = wasmtime.Store()
component = hello_world_rust.Root(store)
print(component.greet(store, "Hello", True))
