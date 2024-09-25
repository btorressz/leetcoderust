impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let current_parts: Vec<i32> = current.split(':')
            .map(|x| x.parse().unwrap())
            .collect();
        let correct_parts: Vec<i32> = correct.split(':')
            .map(|x| x.parse().unwrap())
            .collect();

        let current_minutes = current_parts[0] * 60 + current_parts[1];
        let correct_minutes = correct_parts[0] * 60 + correct_parts[1];

        let mut difference = correct_minutes - current_minutes; 

        let mut operations = 0;
        for &increment in &[60, 15, 5, 1] {
            operations += difference / increment;
            difference %= increment;
        }

        operations
    }
}
