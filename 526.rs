//526. Beautiful Arrangement

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let mut res = 0;
        let mut d: Vec<i32> = vec![1; n as usize]; // Initialize dictionary where 1 means unused

        fn rec(ind: usize, n: i32, d: &mut Vec<i32>, res: &mut i32) {
            // If all numbers are placed, count this arrangement
            if ind > n as usize {
                *res += 1;
                return;
            }

            for i in 1..=n as usize {
                // Try number i if available and if the division rule is satisfied
                if d[i - 1] == 1 && (i % ind == 0 || ind % i == 0) {
                    d[i - 1] = 0;  // Mark i as used
                    rec(ind + 1, n, d, res);  // Recur to place next number
                    d[i - 1] = 1;  // Undo the choice for backtracking
                }
            }
        }

        rec(1, n, &mut d, &mut res); // Start recursion from index 1
        res // Return the result
    }
}
