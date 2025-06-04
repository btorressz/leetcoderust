//1399. Count Largest Group

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        fn sum_of_digits(mut num: i32) -> i32 {
            let mut sum = 0;
            while num > 0 {
                sum += num % 10;
                num /= 10;
            }
            sum
        }

        let mut sum_digits = vec![0; (n + 1) as usize];
        let mut max_sum = 0;

        for i in 1..=n {
            let digit_sum = if i < 10 {
                i
            } else {
                sum_of_digits(i)
            };
            sum_digits[i as usize] = digit_sum;
            max_sum = max_sum.max(digit_sum);
        }

        let mut freq = vec![0; (max_sum + 1) as usize];

        for &sum in &sum_digits[1..] {
            freq[sum as usize] += 1;
        }

        let max_freq = *freq.iter().max().unwrap_or(&0);
        freq.iter().filter(|&&f| f == max_freq).count() as i32
    }
}
