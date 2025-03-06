use wasm_bindgen::prelude::*;
use xxhash_rust::xxh3;

// ================================================= [xxh3] =================================================
#[wasm_bindgen]
pub fn xxh3_64(input: &[u8]) -> u64 {
  xxh3::xxh3_64(input)
}

#[wasm_bindgen]
pub fn xxh3_128(input: &[u8]) -> u128 {
  xxh3::xxh3_128(input)
}
