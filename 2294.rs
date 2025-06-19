//2294. Partition Array Such That Maximum Difference Is K

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        
        let mut count = 1;
        let mut min = nums[0];
        
        for &num in nums.iter().skip(1) {
            if num - min > k {
                count += 1;
                min = num;
            }
        }

        count
    }
}
