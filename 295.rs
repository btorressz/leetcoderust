use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
   max_heap: BinaryHeap<i32>,                // Max-heap for the smaller half
    min_heap: BinaryHeap<Reverse<i32>>,       // Min-heap for the larger half

}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {

    fn new() -> Self {
        MedianFinder {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }
    
      fn add_num(&mut self, num: i32) {
        // add to max-heap first
        self.max_heap.push(num);

        // balance the heaps: move the largest element from max-heap to min-heap
        if let Some(max_top) = self.max_heap.pop() {
            self.min_heap.push(Reverse(max_top));
        }

        // ensure the size invariant: max-heap can have at most one more element than min-heap
        if self.max_heap.len() < self.min_heap.len() {
            if let Some(Reverse(min_top)) = self.min_heap.pop() {
                self.max_heap.push(min_top);
            }
        }
    }
    
fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            // if max-heap is larger, the median is its root
            *self.max_heap.peek().unwrap() as f64
        } else {
            // otherwise, the median is the average of the roots of the two heaps
            let max_top = *self.max_heap.peek().unwrap();
            let min_top = self.min_heap.peek().unwrap().0; // reverse wrapper
            (max_top as f64 + min_top as f64) / 2.0
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
