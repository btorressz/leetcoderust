impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut combination, &mut result);
        result
    }

    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        combination: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>
    ) {
        if target == 0 {
            result.push(combination.clone());
            return;
        }
        if target < 0 {
            return; 
        }

        for i in start..candidates.len() {
            combination.push(candidates[i]); 
            Self::backtrack(candidates, target - candidates[i], i, combination, result); // Recur with updated target
            combination.pop(); 
        }
    }
}
