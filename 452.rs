//452. Minimum Number of Arrows to Burst Balloons

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_by_key(|x| (x[1], x[0])); // Sort by the second value

        let mut i = 0;
        let mut arrwcnt = 0;

        while i < points.len() {
            let mut lg = points[i][1]; // Last position of the current balloon
            while i < points.len() && points[i][0] <= lg {
                i += 1; // Move to the next balloon that can be burst
            }
            arrwcnt += 1; // We need one more arrow
        }

        arrwcnt // Return the total number of arrows
    }
}
