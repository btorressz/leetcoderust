//1524. Number of Sub-arrays With Odd Sum

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut count = [1, 0]; 
        let mut res = 0;
        let mut sum = 0;

        for &x in &arr {
            sum += x;
            res = (res + count[(sum & 1) as usize ^ 1]) % MOD;
            count[(sum & 1) as usize] += 1;
        }

        res
    }
}
