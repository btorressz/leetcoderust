//668. Kth Smallest Number in Multiplication Table

impl Solution {
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        let (mut left, mut right) = (1, m * n);

        while left < right {
            let mid = (left + right) / 2;
            let mut count = 0;

            // Count how many numbers in the multiplication table are ≤ mid
            for i in 1..=m {
                count += (mid / i).min(n);
            }

            if count >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}
