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

fn list_combinations(list: &[i64], current: i64) -> Vec<i64> {
	let mut results = Vec::new();
	for (index, &int) in list.iter().enumerate() {
		results.push(current + int);

		if (list.len() - index) > 1 {
			results.extend(list_combinations(&list[index+1..list.len()], current + int));
		}
	}
	return results;
}

fn subset_sum_rec(mut nums: &[i32], total: i32) -> bool {
    if total == 0 {return true;}
    if total <  0 {return false;}
    let length = nums.len();
    if length == 0{
        return {total == 0}
    }
    let new_array = nums.clone();
    subset_sum_rec(&new_array[1..length], total-new_array[0]) || subset_sum_rec(nums,total);
    return false;
}


#[wasm_bindgen]
pub fn subset_sum(nums_obj: &JsValue, total: i32) -> bool {
    let nums: &[i32] = nums_obj.into_serde().unwrap();
    return subset_sum_rec(nums, total);
}


