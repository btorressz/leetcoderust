// 997. Find the Town Judge
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        if n == 1 {
            return 1; 
        }
        let mut trust_score = vec![0; n as usize + 1]; 
        for t in trust {
            trust_score[t[0] as usize] -= 1; 
            trust_score[t[1] as usize] += 1; 
        }
        for i in 1..=n as usize {
            if trust_score[i] == n - 1 {
                return i as i32; 
            }
        } 
        -1 
    }
}
