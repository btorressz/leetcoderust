//480. Sliding Window Median

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct MedianFinder {
    small: BinaryHeap<i32>,             // max-heap
    large: BinaryHeap<Reverse<i32>>,    // min-heap
    delayed: HashMap<i32, i32>,
    small_size: i32,
    large_size: i32,
    k: usize,
}

impl MedianFinder {
    fn new(k: usize) -> Self {
        Self {
            small: BinaryHeap::new(),
            large: BinaryHeap::new(),
            delayed: HashMap::new(),
            small_size: 0,
            large_size: 0,
            k,
        }
    }

    fn add_num(&mut self, num: i32) {
        if self.small.peek().map_or(true, |&x| num <= x) {
            self.small.push(num);
            self.small_size += 1;
        } else {
            self.large.push(Reverse(num));
            self.large_size += 1;
        }
        self.rebalance();
    }

    fn find_median(&self) -> f64 {
        if self.k % 2 == 1 {
            *self.small.peek().unwrap() as f64
        } else {
            (*self.small.peek().unwrap() as f64 + self.large.peek().unwrap().0 as f64) / 2.0
        }
    }

    fn remove_num(&mut self, num: i32) {
        *self.delayed.entry(num).or_insert(0) += 1;
        if self.small.peek().map_or(false, |&x| num <= x) {
            self.small_size -= 1;
            self.prune_small();
        } else {
            self.large_size -= 1;
            self.prune_large();
        }
        self.rebalance();
    }

    fn prune_small(&mut self) {
        while let Some(&top) = self.small.peek() {
            if let Some(count) = self.delayed.get(&top) {
                self.small.pop();
                if *count == 1 {
                    self.delayed.remove(&top);
                } else {
                    self.delayed.insert(top, count - 1);
                }
            } else {
                break;
            }
        }
    }

    fn prune_large(&mut self) {
        while let Some(&Reverse(top)) = self.large.peek() {
            if let Some(count) = self.delayed.get(&top) {
                self.large.pop();
                if *count == 1 {
                    self.delayed.remove(&top);
                } else {
                    self.delayed.insert(top, count - 1);
                }
            } else {
                break;
            }
        }
    }

    fn rebalance(&mut self) {
        if self.small_size > self.large_size + 1 {
            if let Some(x) = self.small.pop() {
                self.large.push(Reverse(x));
                self.small_size -= 1;
                self.large_size += 1;
                self.prune_small();
            }
        } else if self.small_size < self.large_size {
            if let Some(Reverse(x)) = self.large.pop() {
                self.small.push(x);
                self.large_size -= 1;
                self.small_size += 1;
                self.prune_large();
            }
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let mut ans = Vec::new();
        let mut finder = MedianFinder::new(k as usize);

        for &x in nums.iter().take(k as usize) {
            finder.add_num(x);
        }

        ans.push(finder.find_median());

        for i in k as usize..nums.len() {
            finder.add_num(nums[i]);
            finder.remove_num(nums[i - k as usize]);
            ans.push(finder.find_median());
        }

        ans
    }
}
