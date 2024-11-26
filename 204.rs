impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        // No primes <= 2
        if n <= 2 {
            return 0;
        }

        let n = n as usize;

        // Create a vector to track primes
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        is_prime[1] = false;

        // Mark non-primes
        for i in 2..((n as f64).sqrt() as usize + 1) {
            if is_prime[i] {
                for j in (i * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        // Count primes
        is_prime.into_iter().filter(|&x| x).count() as i32
    }
}
