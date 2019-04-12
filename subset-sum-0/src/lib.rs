mod utils;
extern crate serde_json;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate serde_derive;

#[test]c
#[wasm_bindgen]
pub fn subset_sum_r(array: JsValue){
    let numbers: Vec<u32> = array.into_serde().unwrap();
    const A: u32 = numbers.iter().filter(|x| x.is_positive()).sum();
    const B: u32 = numbers.iter().filter(|x| x.is_negative()).sum();
    assert!(A.is_positive());
    assert!(B.is_negative());
}
