//3100. Water Bottles II

impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut res = num_bottles;

        while num_bottles >= num_exchange {
            num_bottles -= num_exchange;
            num_exchange += 1;
            res += 1;
            num_bottles += 1;
        }

        res
    }
}
