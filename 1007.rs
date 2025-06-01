//1007. Minimum Domino Rotations For Equal Row

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        use std::cmp::{min, Ordering};

        let n = min(tops.len(), bottoms.len());
        let x = tops[0];
        let y = bottoms[0];
        let mut same = 0;
        let mut ansx = 0;
        let mut ansy = 0;
        const INF: i32 = i32::MAX;

        for i in 0..n {
            if tops[i] == bottoms[i] {
                same += 1;
            }
        }

        for i in 1..n {
            if ansx != INF {
                if tops[i] != x && bottoms[i] != x {
                    ansx = INF;
                } else if tops[i] != x && bottoms[i] == x {
                    ansx += 1;
                }
            }
        }

        for i in 1..n {
            if ansy != INF {
                if tops[i] != y && bottoms[i] != y {
                    ansy = INF;
                } else if bottoms[i] != y && tops[i] == y {
                    ansy += 1;
                }
            }
        }

        if ansx != INF {
            ansx = min(ansx, (n - same) as i32 - ansx);
        }
        if ansy != INF {
            ansy = min(ansy, (n - same) as i32 - ansy);
        }

        let res = min(ansx, ansy);
        if res == INF {
            -1
        } else {
            res
        }
    }
}
