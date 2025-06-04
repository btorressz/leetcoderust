//2444. Count Subarrays With Fixed Bounds

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut min_i = -1;
        let mut max_i = -1;
        let mut last = -1;
        let mut res: i64 = 0;

        for (current, &num) in nums.iter().enumerate() {
            let current = current as i32;

            if num < min_k || num > max_k {
                last = current;
            }

            if num == min_k {
                min_i = current;
            }

            if num == max_k {
                max_i = current;
            }

            if min_i != -1 && max_i != -1 {
                let smaller = min_i.min(max_i);
                if smaller > last {
                    res += (smaller - last) as i64;
                }
            }
        }

        res
    }
}
