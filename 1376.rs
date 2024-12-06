//1376. Time Needed to Inform All Employees

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        //  Build the graph
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        
        // Building the graph where manager[i] -> employee[i]
        for (i, &mgr) in manager.iter().enumerate() {
            graph.entry(mgr).or_insert_with(Vec::new).push(i as i32);
        }

        // Initialize the BFS queue and max time variable
        let mut queue = VecDeque::new();
        queue.push_back((head_id, 0)); // (current node, current time)
        
        let mut max_time = 0;

        // Perform BFS
        while let Some((node, curr_time)) = queue.pop_front() {
            // Update max_time if the current time is larger
            max_time = max_time.max(curr_time);
            
            // Enqueue all employees with updated time
            if let Some(employees) = graph.get(&node) {
                for &emplee in employees {
                    queue.push_back((emplee, curr_time + inform_time[node as usize]));
                }
            }
        }

        max_time
    }
}
