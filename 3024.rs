//3024. Type of Triangle

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];

        if a + b > c && b + c > a && a + c > b {
            if a == b && b == c {
                return "equilateral".to_string();
            } else if a != b && b != c && c != a {
                return "scalene".to_string();
            } else {
                return "isosceles".to_string();
            }
        }

        "none".to_string()
    }
}
