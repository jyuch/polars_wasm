use polars::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_vector(x: Vec<i32>, y: Vec<i32>) -> Option<Vec<i32>> {
  let x: Series = x.iter().collect();
  let y: Series = y.iter().collect();

  let result = (&x + &y).i32().ok()?.into_no_null_iter().collect();
  Some(result)
}
