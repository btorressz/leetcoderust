impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        //Map digits to letters
        let digit_to_chars: Vec<&[char]> = vec![
            &[],        // 0
            &[],        // 1
            &['a', 'b', 'c'], // 2
            &['d', 'e', 'f'], // 3
            &['g', 'h', 'i'], // 4
            &['j', 'k', 'l'], // 5
            &['m', 'n', 'o'], // 6
            &['p', 'q', 'r', 's'], // 7
            &['t', 'u', 'v'], // 8
            &['w', 'x', 'y', 'z'], // 9
        ];

        // Prepare result vector
        let mut result = Vec::new();

        // Handle edge case of empty input
        if digits.is_empty() {
            return result;
        }

        // Start backtracking
        let mut combination = String::new();
        Self::backtrack(&digits, 0, &digit_to_chars, &mut combination, &mut result);

        result
    }

    fn backtrack(
        digits: &str,
        index: usize,
        digit_to_chars: &[&[char]],
        combination: &mut String,
        result: &mut Vec<String>
    ) {
        // If the combination length equals the digits length, we found a valid combination
        if index == digits.len() {
            result.push(combination.clone());
            return;
        }

        // Get the current digit and its corresponding characters
        let digit = digits.chars().nth(index).unwrap();
        let chars = digit_to_chars[digit.to_digit(10).unwrap() as usize];

        // Explore each letter for the current digit
        for &ch in chars {
            combination.push(ch); // Add the letter to the combination
            Self::backtrack(digits, index + 1, digit_to_chars, combination, result); // Recurse
            combination.pop(); // Backtrack
        }
    }
}
