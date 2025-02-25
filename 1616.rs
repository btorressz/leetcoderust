//1616. Split Two Strings to Make Palindrome

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        let arr1: Vec<char> = a.chars().collect();
        let arr2: Vec<char> = b.chars().collect();
        Solution::check(&arr1, &arr2) || Solution::check(&arr2, &arr1)
    }

    fn check(arr1: &[char], arr2: &[char]) -> bool {
        let mut l = 0;
        let mut r = arr1.len() - 1;

        while l < r {
            if arr1[l] == arr2[r] {
                l += 1;
                r -= 1;
            } else {
                break;
            }
        }

        if l >= arr1.len() / 2 {
            return true;
        }

        Solution::is_palindrome(arr1, l, r) || Solution::is_palindrome(arr2, l, r)
    }

    fn is_palindrome(arr: &[char], mut l: usize, mut r: usize) -> bool {
        while l < r {
            if arr[l] != arr[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
