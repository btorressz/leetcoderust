//479. Largest Palindrome Product
//SUCCESSFUL ATTEMPT
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9; 
        }

        let upper = 10_i64.pow(n as u32) - 1; 
        let lower = 10_i64.pow((n - 1) as u32); 

        for a in (lower..=upper).rev() {
            let mut palindrome = a;
            let mut b = a;
            
            while b > 0 {
                palindrome = palindrome * 10 + b % 10;
                b /= 10;
            }

            for x in (lower..=upper).rev() {
                if palindrome / x > upper {
                    break; 
                }
                if palindrome % x == 0 {
                    return (palindrome % 1337) as i32;
                }
            }
        }

        9 
    }
}

//ATTEMPT ONE: 
/* impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9; 
        }

        let mx = 10_i64.pow(n as u32) - 1; 

        for a in (mx..=mx / 10).rev() {
            let mut b = a;
            let mut x = a; 
            
            while b > 0 {
                x = x * 10 + b % 10;
                b /= 10;
            }

            let mut t = mx;
            while t * t >= x {
                if x % t == 0 {
                    return (x % 1337) as i32; 
                }
                t -= 1;
            }
        }

        9 
    }
}
*/
