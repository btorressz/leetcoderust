//1200. Minimum Absolute Difference

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();

        let mut min_diff = i32::MAX;
        for i in 1..arr.len() {
            min_diff = min_diff.min(arr[i] - arr[i - 1]);
        }

        let mut res = Vec::new();
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] == min_diff {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }

        res
    }
}
