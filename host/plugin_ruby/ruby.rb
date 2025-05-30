require "wasmtime"

# Generally, you only need a single engine and can re-use it throughout your program.
engine = Wasmtime::Engine.new

# Used to instantiate Components, to link components together, and supply host functionality to components. 
# Imports (host functions) are defined in the linker.
linker = Wasmtime::Component::Linker.new(engine)

# A compiled WebAssembly Component that is ready to be instantiated through a Linker.
# Components are safe to share across threads.
component = Wasmtime::Component::Component.from_file(engine, "hello_world_rust.wasm")

# A Store is a collection of WebAssembly instances and host-defined state.
# All WebAssembly instances will be attached to and refer to a Store.
# A store can keep state to be re-used in Funcs.
# A Store is intended to be a short-lived object in a program.
store = Wasmtime::Store.new(engine)

# An instantiated Component.
# Instances have exports which can be accessed through functions.
# Instances are owned by a Store.
instance = linker.instantiate(store, component)

greet = instance.get_func(["pdureau:wasm-demo/greeter", "greet"])
puts(greet.call("ruby people", false))

