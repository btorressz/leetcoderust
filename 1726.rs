//1726. Tuple with Same Product

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let mut product_count = HashMap::new();
        let mut result = 0;

        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                let product = nums[i] * nums[j];
                *product_count.entry(product).or_insert(0) += 1;
            }
        }

        for &count in product_count.values() {
            if count > 1 {
                result += count * (count - 1) * 4;
            }
        }

        result
    }
}
