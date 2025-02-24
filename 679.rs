//679. 24 Game

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        fn dfs(nums: &mut Vec<f64>) -> bool {
            if nums.len() == 1 {
                return (nums[0] - 24.0).abs() < 1e-6;
            }

            for i in 0..nums.len() {
                for j in 0..nums.len() {
                    if i == j {
                        continue;
                    }

                    let mut next_nums = Vec::new();
                    for k in 0..nums.len() {
                        if k != i && k != j {
                            next_nums.push(nums[k]);
                        }
                    }

                    let a = nums[i];
                    let b = nums[j];

                    let mut operations = vec![
                        a + b,
                        a - b,
                        b - a,
                        a * b,
                    ];

                    if b.abs() > 1e-6 {
                        operations.push(a / b);
                    }
                    if a.abs() > 1e-6 {
                        operations.push(b / a);
                    }

                    for &res in &operations {
                        next_nums.push(res);
                        if dfs(&mut next_nums) {
                            return true;
                        }
                        next_nums.pop();
                    }
                }
            }

            false
        }

        let mut nums = cards.into_iter().map(|x| x as f64).collect::<Vec<f64>>();
        dfs(&mut nums)
    }
}
