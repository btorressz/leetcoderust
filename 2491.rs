impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let total_skill: i32 = skill.iter().sum();
        let target_team_skill = total_skill / (n / 2) as i32;

        let mut skill = skill;
        skill.sort();

        let mut total_chemistry: i64 = 0;

        for i in 0..(n / 2) {
            let team_skill = skill[i] + skill[n - 1 - i];
            if team_skill != target_team_skill {
                return -1; 
            }
            total_chemistry += (skill[i] as i64) * (skill[n - 1 - i] as i64);
        }

        total_chemistry
    }
}
