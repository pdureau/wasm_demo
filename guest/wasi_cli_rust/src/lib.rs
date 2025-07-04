#[allow(warnings)]
mod bindings;

// Imports (host functions)
use bindings::pdureau::wasm_demo::greeter;
use bindings::wasi::cli::environment;
// Exports (guest functions)
use bindings::exports::wasi::cli::run::Guest as RunInterface;

struct Component;

impl RunInterface for Component {
    fn run() -> Result<(), ()> {
        let args: Vec<String> = environment::get_arguments();
        let name = &args[1];
        let up: bool = match args[2].as_str() {
            "true" => true,
            "false" => false,
            _ => false,
        };
        println!("{}", greeter::greet(name, up));
        Ok(())
    }
}

bindings::export!(Component with_types_in bindings);
