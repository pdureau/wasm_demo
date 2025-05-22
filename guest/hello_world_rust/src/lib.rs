#[allow(warnings)]
mod bindings;

// bindings::exports::{package.namespace}::{package.id}::{interface}::Guest;
// With: "package pdureau:wasm-demo;" & "world greeter {" 
use bindings::exports::pdureau::wasm_demo::greeter::Guest as Greeter;

struct Component;

impl Greeter for Component {
    fn greet(name: String, up: bool) -> String {
        let mut greeting = format!("Hello, {name}");
        if up {
            greeting = greeting.to_uppercase()
        }
        greeting
    }
}

bindings::export!(Component with_types_in bindings);
