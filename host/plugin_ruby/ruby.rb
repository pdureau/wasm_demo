require "wasmtime"

engine = Wasmtime::Engine.new

puts Dir.pwd

linker = Wasmtime::Component::Linker.new(engine)
component = Wasmtime::Component::Component.from_file(engine, "hello_world_rust.wasm")
store = Wasmtime::Store.new(engine)
instance = linker.instantiate(store, component)

instance.invoke("greet", "ruby people", false)

