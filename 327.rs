//327. Count of Range Sum
struct BinaryIndexedTree {
    n: usize,
    c: Vec<i32>,
}

impl BinaryIndexedTree {
    // Constructor
    fn new(n: usize) -> Self {
        Self {
            n,
            c: vec![0; n + 1],
        }
    }

    // Update function
    fn update(&mut self, mut x: usize, v: i32) {
        while x <= self.n {
            self.c[x] += v;
            x += x & (!x + 1); // x += x & -x
        }
    }

    // Query function
    fn query(&self, mut x: usize) -> i32 {
        let mut s = 0;
        while x > 0 {
            s += self.c[x];
            x -= x & (!x + 1); // x -= x & -x
        }
        s
    }
}

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for i in 0..n {
            s[i + 1] = s[i] + nums[i] as i64; // Use i64 to avoid overflow
        }

        // Create a list of unique values for coordinate compression
        let mut arr = vec![0; (n + 1) * 3];
        for (i, &x) in s.iter().enumerate() {
            arr[i * 3] = x;
            arr[i * 3 + 1] = x - lower as i64;
            arr[i * 3 + 2] = x - upper as i64;
        }
        arr.sort_unstable();
        arr.dedup(); // Remove duplicates

        // Create a BIT
        let m = arr.len();
        let mut tree = BinaryIndexedTree::new(m);

        // Coordinate compression + querying
        let mut ans = 0;
        for &x in &s {
            let l = Self::search(&arr, x - upper as i64);
            let r = Self::search(&arr, x - lower as i64);
            ans += tree.query(r) - tree.query(l - 1);
            let idx = Self::search(&arr, x);
            tree.update(idx, 1);
        }
        ans
    }

    // Binary search for coordinate compression
    fn search(nums: &Vec<i64>, x: i64) -> usize {
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] >= x {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        l + 1 // BIT uses 1-based indexing
    }
}
