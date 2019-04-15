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

fn subset_sum<NumType: Num+PartialOrd, IterType: Iterator<NumType> + Clone>
          (mut nums: IterType, total: NumType) -> bool {
    let zero = std::num::Zero::zero();
    if total == zero {return true;}
    if total <  zero {return false;}
    match nums.next() {
        None => total == zero,
        Some(x) => {
            subset_sum(nums.clone(), total-x) || subset_sum(nums,total)
        }
    }
}

#[wasm_bindgen]
pub fn subset_sum_r(array: JsValue){
    let numbers: Vec<i32> = array.into_serde().unwrap();
    const A: i32 = numbers.iter().filter(|x| x.is_positive()).sum();
    const B: i32 = numbers.iter().filter(|x| x.is_negative()).sum();
    assert!(A.is_positive());
    assert!(B.is_negative());
}


