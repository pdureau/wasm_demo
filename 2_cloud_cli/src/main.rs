mod bindings;
use bindings::Guest;
use std::env;

bindings::export!(Component with_types_in bindings);

struct Component;

impl Guest for Component {
    fn greet(name: String, up: bool) -> String {
        let mut greeting = format!("Hello, {name}");
        if up {
            greeting = greeting.to_uppercase()
        }
        greeting
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[1];
    let up: bool = match &args[2][..] {
        "true" => true,
        "false" => false,
        _ => false,
    };
    println!("{}", Component::greet(name.to_owned(), up));
}
