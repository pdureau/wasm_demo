#[allow(warnings)]
mod bindings;

use bindings::exports::pdureau::wasm_demo::greeter::Guest as GreeterInterface;

struct Component;

impl GreeterInterface for Component {
    fn greet(name: String, up: bool) -> String {
        let mut greeting = format!("Hello, {name}");
        if up {
            greeting = greeting.to_uppercase()
        }
        greeting
    }
}

bindings::export!(Component with_types_in bindings);
