pub struct NumArray {
    segment_tree: Vec<i32>, // Segment tree array
    nums: Vec<i32>,         // Original array to store values
    n: usize,               // size of the array
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut segment_tree = vec![0; 4 * n]; // allocate space for the segment tree
        let mut obj = Self {
            segment_tree,
            nums,
            n,
        };
        obj.build_tree(0, 0, n - 1); // build the segment tree
        obj
    }

    // build the segment tree from the nums array
    fn build_tree(&mut self, node: usize, start: usize, end: usize) {
        if start == end {
            // leaf node stores the original array value
            self.segment_tree[node] = self.nums[start];
        } else {
            let mid = (start + end) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;

            // Recursively build the left and right subtrees
            self.build_tree(left_child, start, mid);
            self.build_tree(right_child, mid + 1, end);

            // Store the sum of the left and right children in the current node
            self.segment_tree[node] =
                self.segment_tree[left_child] + self.segment_tree[right_child];
        }
    }

    // update the value at a specific index
    pub fn update(&mut self, index: i32, val: i32) {
        self.update_tree(0, 0, self.n - 1, index as usize, val);
    }

    fn update_tree(&mut self, node: usize, start: usize, end: usize, idx: usize, val: i32) {
        if start == end {
            // update the leaf node
            self.segment_tree[node] = val;
            self.nums[idx] = val;
        } else {
            let mid = (start + end) / 2;
            let left_child = 2 * node + 1;
            let right_child = 2 * node + 2;

            if idx <= mid {
                // if the index is in the left subtree
                self.update_tree(left_child, start, mid, idx, val);
            } else {
                // if the index is in the right subtree
                self.update_tree(right_child, mid + 1, end, idx, val);
            }

            // update the current node after updating the child
            self.segment_tree[node] =
                self.segment_tree[left_child] + self.segment_tree[right_child];
        }
    }

    // Query the sum of a range
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.query_tree(0, 0, self.n - 1, left as usize, right as usize)
    }

    fn query_tree(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        if l > end || r < start {
            // Range is completely outside the current node
            return 0;
        }

        if l <= start && end <= r {
            // Range is completely inside the current node
            return self.segment_tree[node];
        }

        // Partial overlap: query both left and right children
        let mid = (start + end) / 2;
        let left_sum = self.query_tree(2 * node + 1, start, mid, l, r);
        let right_sum = self.query_tree(2 * node + 2, mid + 1, end, l, r);

        left_sum + right_sum
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */
