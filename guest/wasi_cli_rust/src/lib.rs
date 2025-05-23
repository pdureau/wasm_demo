#[allow(warnings)]
mod bindings;
use bindings::exports::pdureau::wasm_demo::greeter::Guest as GreeterInterface;
use bindings::exports::wasi::cli::run::Guest as RunInterface;
use std::env;

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

impl RunInterface for Component {
    fn run() -> Result<(), ()> {
        let args: Vec<String> = env::args().collect();
        let name = &args[1];
        let up: bool = match &args[2][..] {
            "true" => true,
            "false" => false,
            _ => false,
        };
        println!("{}", Component::greet(name.to_owned(), up));
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
