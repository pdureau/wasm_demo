#[allow(warnings)]
mod bindings;

use bindings::exports::pdureau::wasm_demo::greeter::Guest as Greeter;
use bindings::exports::wasi::http::incoming_handler::Guest as IncomingHandler;
pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

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

impl IncomingHandler for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().unwrap();
        ResponseOutparam::set(outparam, Ok(resp));
        let out = body.write().unwrap();
        out.blocking_write_and_flush(Component::greet("Brussels".to_string(), false).as_bytes())
            .unwrap();
        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
