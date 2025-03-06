use wasm_bindgen::prelude::*;
use xxhash_rust::xxh3;

// ================================================= [xxh3] =================================================
#[wasm_bindgen]
pub fn xxh3_64(input: &str) -> u64 {
  xxh3::xxh3_64(input.as_bytes())
}

#[wasm_bindgen]
pub fn xxh3_128(input: &str) -> u128 {
  xxh3::xxh3_128(input.as_bytes())
}
