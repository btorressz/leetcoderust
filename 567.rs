use std::collections::HashMap;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let len1 = s1.len();
        let len2 = s2.len();

        if len1 > len2 {
            return false; 
        }

        let mut count1 = [0; 26]; 
        let mut count2 = [0; 26]; 

        for ch in s1.chars() {
            count1[(ch as usize - 'a' as usize)] += 1;
        }

        for ch in s2.chars().take(len1) {
            count2[(ch as usize - 'a' as usize)] += 1;
        }

        if count1 == count2 {
            return true;
        }

        for i in len1..len2 {
            count2[(s2.chars().nth(i).unwrap() as usize - 'a' as usize)] += 1;
            count2[(s2.chars().nth(i - len1).unwrap() as usize - 'a' as usize)] -= 1;

            if count1 == count2 {
                return true;
            }
        }

        false
    }
}
