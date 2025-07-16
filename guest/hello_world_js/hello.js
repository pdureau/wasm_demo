// Import interface from WIT.
import { reverse } from "pdureau:wasm-demo/formatter";

// Export interface from WIT.
export const greeter = {
  greet: function (name, up, reversed) {
    if (up) {
      name = name.toUpperCase();
    }
    if (reversed) {
      name = reverse(name);
    }
    return `Hello ${name} from JS`;
  },
};
