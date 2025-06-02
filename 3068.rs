//3068. Find the Maximum Sum of Node Values

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let mut tot_sum: i64 = 0;
        let mut count = 0;
        let mut pos_min = i32::MAX;
        let mut neg_max = i32::MIN;

        for &value in &nums {
            let xor_val = value ^ k;
            tot_sum += value as i64;
            let net_change = xor_val - value;

            if net_change > 0 {
                pos_min = pos_min.min(net_change);
                tot_sum += net_change as i64;
                count += 1;
            } else {
                neg_max = neg_max.max(net_change);
            }
        }

        if count % 2 == 0 {
            tot_sum
        } else {
            std::cmp::max(
                tot_sum - pos_min as i64,
                tot_sum + neg_max as i64,
            )
        }
    }
}
