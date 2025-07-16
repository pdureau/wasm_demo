from bindings.hello_world_js import Root, RootImports
from wasmtime import Store

# A Store is a collection of WebAssembly instances and host-defined state.
# Store can keep state to be re-used in Funcs.
# All WebAssembly instances and items will be attached to and refer to a Store.
# A Store is intended to be a short-lived object in a program.
store = Store()


# Define host functions.
class Host:
    def reverse(self, string: str):
        return string[::-1]


root = Root(store, RootImports(Host()))
print(root.greeter().greet(store, "pythonistas", False, True))
