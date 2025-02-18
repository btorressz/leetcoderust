//2375. Construct Smallest Number From DI String

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let size = pattern.len();
        let mut result: Vec<usize> = (1..=size + 1).collect();  

        let chars: Vec<char> = pattern.chars().collect();
        let mut i = 0;

        while i < size {
            let mut t = i;
            while t < size && chars[t] == 'D' {
                t += 1;
            }

            result[i..=t].reverse(); 

            if t != i {
                i = t; 
            } else {
                i += 1;
            }
        }

        result.into_iter().map(|num| num.to_string()).collect::<String>()
    }
}
