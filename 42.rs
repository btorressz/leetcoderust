impl Solution {
    #[allow(dead_code)]
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left: Vec<i32> = vec![0; n];
        let mut right: Vec<i32> = vec![0; n];

        left[0] = height[0];
        right[n - 1] = height[n - 1];

        for i in 1..n {
            left[i] = std::cmp::max(left[i - 1], height[i]);
            right[n - i - 1] = std::cmp::max(right[n - i], height[n - i - 1]);
        }

        let mut ans = 0;

        for i in 0..n {
            ans += std::cmp::min(left[i], right[i]) - height[i];
        }

        ans
    }
}
