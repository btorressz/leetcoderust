//1550. Three Consecutive Odds

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        for i in 2..arr.len() {
            if arr[i] % 2 == 1 && arr[i - 1] % 2 == 1 && arr[i - 2] % 2 == 1 {
                return true;
            }
        }
        false
    }
}
