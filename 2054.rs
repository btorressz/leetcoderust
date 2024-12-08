/*ATTEMPT Three:WRONG ANSWER :( 
use std::cmp::max;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        //  Sort the events by end time
        let mut events = events;
        events.sort_by_key(|e| e[1]);
        
        let n = events.len();
        
        //  Create dp array where dp[i] holds the best sum achievable from event i and later
        let mut dp = vec![0; n];
        
        // Initialize dp: the best value for the last event is its own value
        dp[n - 1] = events[n - 1][2];
        
        // Fill the dp array from right to left (from last event to first)
        for i in (0..n - 1).rev() {
            dp[i] = max(dp[i + 1], events[i][2]);
        }
        
        //  Use binary search to find the next valid event for each event
        let mut ans = 0; // To store the final maximum sum
        
        for i in 0..n {
            // Binary search to find the first event that starts after current event ends
            let mut left = i + 1;
            let mut right = n;
            let current_end = events[i][1];
            let mut idx = n;
            
            // Perform binary search
            while left < right {
                let mid = (left + right) / 2;
                if events[mid][0] > current_end {
                    idx = mid;
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            
            // If found a valid event (index < n), add its value to the current event's value
            if idx < n {
                ans = max(ans, events[i][2] + dp[idx]);
            }
            
            // Update the answer with the value of the current event alone
            ans = max(ans, events[i][2]);
        }
        
        //  Return the final answer
        ans
    }
}
*/
/*
ATTEMPT TWO: WRONG ANSWER 
use std::cmp::max;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        // Sort events by the start time
        let mut events = events;
        events.sort_by_key(|e| e[0]);
        let n = events.len();
        // Vector to store the maximum value of the second event after each event
        let mut f = vec![events[n - 1][2]; n];
        // Fill the f array with the maximum value for each event
        for i in (0..n - 1).rev() {
            f[i] = max(f[i + 1], events[i][2]);
        }
        let mut ans = 0;
        // Iterate through each event to compute the maximum possible value
        for i in 0..n {
            let (start, end, mut v) = (events[i][0], events[i][1], events[i][2]);
            // Find the first event that starts after the current event's end time
            let idx = events.binary_search_by_key(&(end + 1), |x| x[0]).unwrap_or(n);
            // If the next event exists, add its value to the current event's value
            if idx < n {
                v += f[idx];
            }
            // Update the max possible value
            ans = max(ans, v);
        }
        ans
    }
}
*/
/*ATTEMPT ONE: WRONG ANSWER
use std::cmp::max;
use std::collections::VecDeque;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        // Sort events by the start time (first value in the list)
        let mut events = events;
        events.sort_by_key(|e| e[0]);
        let n = events.len();
        
        // Vector to store the maximum value of the second event after each event
        let mut f = vec![events[n - 1][2]; n];
        // Fill the f array with the maximum value for each event
        for i in (0..n - 1).rev() {
            f[i] = max(f[i + 1], events[i][2]);
        }
        let mut ans = 0;
        // Iterate through each event
        for event in events.iter() {
            let (_, e, mut v) = (event[0], event[1], event[2]);
            // Find the first event that starts after the current event's end
            let idx = events.binary_search_by_key(&e, |x| x[0]).unwrap_or(n);
            if idx < n {
                v += f[idx];
            }
            ans = max(ans, v);
        }
        ans
    }
}
*/
