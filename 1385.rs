//1385. Find the Distance Value Between Two Arrays
//ATTEMPT TWO:SUCCESSFUL
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        // Sort arr2 for binary search
        let mut arr2 = arr2;
        arr2.sort();

        let mut count = 0;

        // Iterate over each value in arr1
        for &num1 in &arr1 {
            // Find the insertion point in sorted arr2
            let ind = arr2.binary_search(&num1).unwrap_or_else(|x| x);

            // Flag to check if num1 is too close to any value in arr2
            let mut left_far = true;  // Check if distance from left neighbor is greater than d
            let mut right_far = true; // Check if distance from right neighbor is greater than d

            // Check the left value, if it exists
            if ind > 0 && (num1 - arr2[ind - 1]).abs() <= d {
                left_far = false;
            }
            // Check the right value, if it exists
            if ind < arr2.len() && (num1 - arr2[ind]).abs() <= d {
                right_far = false;
            }

            // If both the left and right value are far enough, increment count
            if left_far && right_far {
                count += 1;
            }
        }

        count
    }
}



/*
ATTEMPT ONE: WRONG ANSWER
impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        // Sort arr2 for binary search
        let mut arr2 = arr2;
        arr2.sort();

        let mut count = 0;

        // Iterate over each value in arr1
        for &num1 in &arr1 {
            // Find the insertion point in sorted arr2
            let ind = arr2.binary_search(&num1).unwrap_or_else(|x| x);

            // If num1 is larger than all values in arr2, check distance to the last value
            if ind == arr2.len() {
                if (arr2[arr2.len() - 1] - num1).abs() > d {
                    count += 1;
                }
            } else {
                // Check the distance to the nearest values in arr2
                let left_far = ind > 0 && (arr2[ind - 1] - num1).abs() > d;
                let right_far = (arr2[ind] - num1).abs() > d;

                if left_far && right_far {
                    count += 1;
                }
            }
        }

        count
    }
}


*/
