struct MyCalendar {
    events: Vec<(i32, i32)>, 
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            events: Vec::new(),
        }
    }
    
    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.events {
            if start < e && s < end { 
                return false;
            }
        }
        
        self.events.push((start, end));
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let mut obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

