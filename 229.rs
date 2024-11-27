//229. Majority Element II

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut element1 = 0;
        let mut element2 = 0;
        let mut count1 = 0;
        let mut count2 = 0;

        for &num in &nums {
            if num == element1 {
                count1 += 1;
            } else if num == element2 {
                count2 += 1;
            } else if count1 == 0 {
                element1 = num;
                count1 = 1;
            } else if count2 == 0 {
                element2 = num;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        let mut result = Vec::new();
        let mut freq1 = 0;
        let mut freq2 = 0;
        let threshold = nums.len() / 3;

        for &num in &nums {
            if num == element1 {
                freq1 += 1;
            } else if num == element2 {
                freq2 += 1;
            }
        }

        if freq1 > threshold {
            result.push(element1);
        }
        if freq2 > threshold {
            result.push(element2);
        }

        result
    }
}
