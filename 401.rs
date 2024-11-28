//401. Binary Watch

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on > 10 {
            return vec![]; 
        }

        fn generate_numbers(count: i32, max_value: i32) -> Vec<i32> {
            (0..=max_value)
                .filter(|&x| x.count_ones() as i32 == count)
                .collect()
        }

        let mut result = Vec::new();

        for h in 0..=turned_on {
            let m = turned_on - h; 

            let hours = generate_numbers(h, 11); 
            let minutes = generate_numbers(m, 59); 

            for &hour in &hours {
                for &minute in &minutes {
                    result.push(format!("{}:{:02}", hour, minute));
                }
            }
        }

        result
    }
}
