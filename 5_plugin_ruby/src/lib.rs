mod bindings;
use bindings::Guest;

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
