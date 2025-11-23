from wasmtime import Store
from wasmtime.component import Instance
from utils import get_export_func, init_instance


# Store: Runtime state container holding instances, memories, globals, and execution context.
store = Store()
# Instance: Live instantiation of a component with allocated memory, globals, and callable functions
instance: Instance = init_instance(store, "../../hello_world_rust.wasm", {})

greet = get_export_func(store, instance, "pdureau:wasm-demo/greeter", "greet")
if greet:
    print(greet(store, "pythonistas", False))
