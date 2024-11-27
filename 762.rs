//762. Prime Number of Set Bits in Binary Representation
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        fn is_prime(n: i32) -> bool {
            if n < 2 {
                return false;
            }
            for i in 2..=((n as f64).sqrt() as i32) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        let primes: Vec<bool> = (0..=20).map(|x| is_prime(x)).collect();

        let mut count = 0;
        for num in left..=right {
            let set_bits = num.count_ones() as usize;
            if primes[set_bits] {
                count += 1;
            }
        }
        count
    }
}
