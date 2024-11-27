//274. H-Index
//Solution 1 : Sorting Based
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_by(|a, b| b.cmp(a));
        
        let mut h = 0;

        for i in 0..citations.len() {
            if citations[i] >= (i + 1) as i32 {
                h = i + 1; 
            } else {
                break;
            }
        }
        h as i32
    }
}

//Solution 2 : Binary Search on Sorted Array
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        let n = citations.len();
        citations.sort();

        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = (left + right) / 2;
            if citations[mid] >= (n - mid) as i32 {
                right = mid; 
            } else {
                left = mid + 1; 
            }
        }

        (n - left) as i32
    }
}

//Solution 3 : Reverse Iteration on Sorted Array
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let n = citations.len();

        for i in 0..n {
            if citations[i] >= (n - i) as i32 {
                return (n - i) as i32;
            }
        }

        0
    }
}

