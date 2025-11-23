from wasmtime import Store
from wasmtime.component import Instance
from utils import get_export_func, init_instance


def reverse(store: Store, string: str) -> str:
    return string[::-1]


host = {"pdureau:wasm-demo/formatter": {"reverse": reverse}}

# Store: Runtime state container holding instances, memories, globals, and execution context.
store = Store()
# Instance: Live instantiation of a component with allocated memory, globals, and callable functions
instance: Instance = init_instance(store, "../../hello_world_imports_rust.wasm", host)

greet = get_export_func(store, instance, "pdureau:wasm-demo/greeter", "greet")
if greet:
    print(greet(store, "pythonistas", False, True))
