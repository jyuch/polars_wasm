import init, { add_vector } from "./pkg/rs_lib.js";

init().then(() => {
  const a = new Int32Array([1, 2, 3]);
  const b = new Int32Array([4, 5, 6]);

  const c = add_vector(a, b);

  console.log(c);
});
