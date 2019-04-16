mod utils;
extern crate serde_json;
extern crate wasm_bindgen;

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

fn subset_sum_rec(mut nums:  Vec<i32>, total: i32) -> bool {
    let length = nums.len();
    let rec_bool;
    if length == 0{
        if total == 0 {return true;}
    }
    let mut new_array = nums.clone();
    let first = new_array.pop();
    if first.is_none(){
        rec_bool = subset_sum_rec(nums,total);
    }else{
        let new_total = total - first.unwrap();
        rec_bool = subset_sum_rec(new_array, new_total) || subset_sum_rec(nums,total)
    }
    if rec_bool {return true;}
    return false;
}


#[wasm_bindgen]
pub fn subset_sum(nums_obj: &JsValue, total: i32) -> bool {
    let nums: Vec<i32> = nums_obj.into_serde().unwrap();
    return subset_sum_rec(nums, total);
}


