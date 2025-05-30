#[allow(warnings)]
mod bindings;
// Imports (host functions)
use bindings::pdureau::wasm_demo::formatter;
// Exports (guest functions)
use bindings::exports::pdureau::wasm_demo::greeter::Guest as GreeterInterface;

struct Component;

impl GreeterInterface for Component {
    fn greet(name: String, up: bool, reversed: bool) -> String {
        let mut greeting = format!("Hello, {name}");
        if up {
            greeting = greeting.to_uppercase();
        }
        if reversed {
            greeting = formatter::reverse(&greeting);
        }
        greeting
    }
}

bindings::export!(Component with_types_in bindings);
