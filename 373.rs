//373. Find K Pairs with Smallest Sums

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new(); // Min-heap using Reverse for smallest elements
        let mut ans = Vec::new();
        let k = k as usize;

        // Initialize the heap with pairs (sum, index in nums1, index in nums2)
        for i in 0..nums1.len().min(k) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        // Extract the k smallest pairs
        while let Some(Reverse((_, i, j))) = heap.pop() {
            ans.push(vec![nums1[i], nums2[j]]);
            if ans.len() == k {
                break;
            }

            // If there's a next element in nums2, push the new pair into the heap
            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        ans
    }
}
/*impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut pairs = Vec::new();

        // Generate all pairs
        for &n1 in &nums1 {
            for &n2 in &nums2 {
                pairs.push((n1 + n2, vec![n1, n2]));
            }
        }

        // Sort pairs by sum
        pairs.sort_by(|a, b| a.0.cmp(&b.0));

        // Take the first k pairs
        pairs.iter().take(k as usize).map(|p| p.1.clone()).collect()
    }
}

*/
