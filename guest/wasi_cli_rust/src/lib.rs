#[allow(warnings)]
mod bindings;

use bindings::exports::pdureau::wasm_demo::greeter::Guest as Greeter;
use bindings::exports::wasi::cli::run::Guest as Run;

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

impl Run for Component {
    fn run() -> Result<(), ()> {
        println!("{}", Component::greet("Linux".to_string(), false));
        Ok(())
    }
}
