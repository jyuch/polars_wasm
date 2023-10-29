use polars::prelude::*;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_vector(x: Vec<i32>, y: Vec<i32>) -> Option<Vec<i32>> {
  let x: Series = x.iter().collect();
  let y: Series = y.iter().collect();

  let result = (&x + &y).i32().ok()?.into_no_null_iter().collect();
  Some(result)
}

#[wasm_bindgen]
pub fn load_csv_and_sum(csv: Vec<u8>) -> Option<i32> {
  let df = CsvReader::new(Cursor::new(csv)).finish().ok()?;
  let df = df
    .clone()
    .lazy()
    .select([col("value").sum()])
    .collect()
    .ok()?;
  Some(df.get_row(0).ok()?.0.iter().nth(0)?.try_extract().ok()?)
}
