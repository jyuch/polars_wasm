import { instantiate } from "./lib/rs_lib.generated.js";

const { add_vector } = await instantiate();

const x = new Int32Array([1, 2, 3]);
const y = new Int32Array([4, 5, 6]);

const c = add_vector(x, y);

console.log(c);
