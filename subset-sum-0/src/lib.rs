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

fn subset_sum(mut nums: &[Option<i64>], total: i64) -> bool {
    if total == 0 {return true;}
    if total <  0 {return false;}
    match nums[0] {
        None => total == 0,
        Some(x) => {
            subset_sum(nums.clone(), total-x) || subset_sum(nums,total)
        }
    }
}



