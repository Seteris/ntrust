mod utils;

use wasm_bindgen::prelude::*;

const I64_MAX: i64 = 9223372036854775807;
const I64_MIN: i64 = -9223372036854775808;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn biggest_signed() -> String {
    I64_MAX.to_string()
}

#[wasm_bindgen]
pub fn biggest_unsigned() -> String {
    I64_MIN.to_string()
}

#[wasm_bindgen]
pub fn get_random_bytes(num_bytes: i64) -> Vec<u8> {
    let mut random: Vec<u8> = vec![0; num_bytes as usize];
    getrandom::getrandom(&mut random).unwrap();
    random
}