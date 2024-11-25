use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;

        // Create an adjacency list to represent the graph
        // Each course points to the list of courses that depend on it
        let mut graph = vec![vec![]; num_courses];

        // Array to store the in-degree of each course (number of prerequisites)
        let mut in_degree = vec![0; num_courses];

        // Build the graph and calculate in-degrees
        for prerequisite in prerequisites {
            let course = prerequisite[0] as usize;  // Course that depends on `prereq`
            let prereq = prerequisite[1] as usize; // Prerequisite course
            graph[prereq].push(course);            // Add an edge `prereq -> course`
            in_degree[course] += 1;                // Increment the in-degree of `course`
        }

        // Create a queue to process courses with no prerequisites (in-degree = 0)
        let mut queue = VecDeque::new();
        for course in 0..num_courses {
            if in_degree[course] == 0 {
                queue.push_back(course); // Add course with no prerequisites to the queue
            }
        }

        // Variable to count how many courses we can process
        let mut processed_courses = 0;

        // Process courses in topological order
        while let Some(course) = queue.pop_front() {
            processed_courses += 1; // Increment the count of processed courses

            // Iterate over all courses that depend on the current course
            for &neighbor in &graph[course] {
                in_degree[neighbor] -= 1; // Reduce the in-degree of the dependent course

                // If the in-degree of a course becomes 0, it can now be processed
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        // If we have processed all courses, return true (no cycle in the graph)
        // Otherwise, return false (there's a cycle preventing completion)
        processed_courses == num_courses
    }
}
