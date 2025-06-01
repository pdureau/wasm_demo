import { greeter } from "./hello_world_imports_rust.js";

let message = greeter.greet("node JS", false, true);
console.log(message);
