use std::collections::BTreeMap;

struct MyCalendarTwo {
    bookings: BTreeMap<i32, i32>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            bookings: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.bookings.entry(start).or_insert(0) += 1;
        *self.bookings.entry(end).or_insert(0) -= 1;

        let mut current_overlaps = 0;

        for &count in self.bookings.values() {
            current_overlaps += count;
            if current_overlaps >= 3 {
                
                *self.bookings.entry(start).or_insert(0) -= 1; 
                *self.bookings.entry(end).or_insert(0) += 1;   
                return false; 
            }
        }

        true 
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let mut obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
