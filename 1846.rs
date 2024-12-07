//1846. Maximum Element After Decreasing and Rearranging
// I used a sorted vector and a greedy algorithm to ensure the array elements are adjusted so that each element is at most 1 greater than the previous one, while maximizing the last element.

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(mut arr: Vec<i32>) -> i32 {
        // Sort the array in ascending order
        arr.sort();

        // Set the first element to 1
        arr[0] = 1;

        // Loop through the array starting from the second element
        for i in 1..arr.len() {
            // If the current element is too large, make it the previous element + 1
            if arr[i] > arr[i - 1] + 1 {
                arr[i] = arr[i - 1] + 1;
            }
        }

        // Return the last element (the largest one after adjustments)
        *arr.last().unwrap()
    }
}
