impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut s1 = num.to_string();
        let mut s2 = num.to_string();
        let n = s1.len();
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();

        let mut c = ' ';
        for &ch in &chars1 {
            if ch != '9' {
                c = ch;
                break;
            }
        }
        if c != ' ' {
            s1 = s1.chars().map(|x| if x == c { '9' } else { x }).collect();
        }

        let mut replaced = false;
        for (i, &ch) in chars2.iter().enumerate() {
            if i == 0 && ch != '1' {
                c = ch;
                s2 = s2.chars().map(|x| if x == c { '1' } else { x }).collect();
                replaced = true;
                break;
            } else if i != 0 && ch != '0' && ch != chars2[0] {
                c = ch;
                s2 = s2.chars().map(|x| if x == c { '0' } else { x }).collect();
                replaced = true;
                break;
            }
        }

        let max_val = s1.parse::<i32>().unwrap();
        let min_val = s2.parse::<i32>().unwrap();
        max_val - min_val
    }
}
