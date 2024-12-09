//853. Car Fleet
// I used a stack to track fleets, sorting cars by position and grouping them by their time to reach the target, 
// where cars with longer times form new fleets and those with shorter times join the earlier fleet.

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n = position.len();
        
        // Create a list of cars with their position and time to reach the target
        let mut cars: Vec<(i32, f64)> = position.into_iter()
            .zip(speed.into_iter())
            .map(|(p, s)| {
                let time = (target as f64 - p as f64) / (s as f64);
                (p, time)
            })
            .collect();
        
        // Sort cars by position (from left to right)
        cars.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Stack to keep track of fleets
        let mut stack = Vec::new();
        let mut pre = 0.0;  // Time of the last fleet
        
        // Go through cars from the farthest to the closest
        for (_, time) in cars.into_iter().rev() {
            // If this car takes longer, it's a new fleet
            if time > pre {
                stack.push(time);  // Add this fleet
                pre = time;  // Update the last fleet time
            }
        }
        
        // The number of fleets is how many are in the stack
        stack.len() as i32
    }
}
