from bindings.hello_world_rust import Root
from wasmtime import Store

# A Store is a collection of WebAssembly instances and host-defined state.
# Store can keep state to be re-used in Funcs.
# All WebAssembly instances and items will be attached to and refer to a Store.
# A Store is intended to be a short-lived object in a program.
store = Store()

root = Root(store)
print(root.greeter().greet(store, "pythonistas", False))
