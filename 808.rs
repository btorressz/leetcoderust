//808. Soup Servings

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n == 0 {
            return 0.5;
        }
        if n >= 4800 {
            return 1.0;
        }

        use std::collections::HashMap;

        fn dp(a: i32, b: i32, memo: &mut HashMap<(i32, i32), f64>) -> f64 {
            if a <= 0 && b <= 0 {
                return 0.5;
            }
            if a <= 0 {
                return 1.0;
            }
            if b <= 0 {
                return 0.0;
            }
            if let Some(&val) = memo.get(&(a, b)) {
                return val;
            }

            let val = 0.25 * (
                dp(a - 100, b, memo) +
                dp(a - 75, b - 25, memo) +
                dp(a - 50, b - 50, memo) +
                dp(a - 25, b - 75, memo)
            );

            memo.insert((a, b), val);
            val
        }

        let mut memo = HashMap::new();
        dp(n, n, &mut memo)
    }
}
