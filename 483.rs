//483. Smallest Good Base

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        fn cal(k: u128, m: u32) -> u128 {
            let mut p = 1_u128;
            let mut s = 1_u128;

            for _ in 0..m {
                p *= k;
                s += p;
            }
            s
        }

        let num: u128 = n.parse().unwrap();
        
        for m in (2..=63).rev() { 
            let (mut left, mut right) = (2, num - 1);

            while left < right {
                let mid = (left + right) / 2;
                if cal(mid, m) >= num {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            if cal(left, m) == num {
                return left.to_string();
            }
        }

        (num - 1).to_string()
    }
}
