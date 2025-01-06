//733. Flood Fill

use std::collections::VecDeque;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let sr = sr as usize;
        let sc = sc as usize;
        let original_color = image[sr][sc];
        
        // If the starting pixel is already the new color, return the image immediately
        if original_color == new_color {
            return image;
        }

        // Directions for up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        
        // Initialize BFS queue
        let mut queue = VecDeque::new();
        queue.push_back((sr, sc));

        // Perform BFS
        while let Some((r, c)) = queue.pop_front() {
            // Change the color of the current pixel
            image[r][c] = new_color;

            // Explore neighbors (up, down, left, right)
            for &(dr, dc) in &directions {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                // Check if the neighbor is within bounds and has the original color
                if nr >= 0 && (nr as usize) < image.len() && 
                   nc >= 0 && (nc as usize) < image[0].len() &&
                   image[nr as usize][nc as usize] == original_color 
                {
                    queue.push_back((nr as usize, nc as usize));
                }
            }
        }

        image
    }
}
