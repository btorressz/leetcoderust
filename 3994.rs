//Attempt one : wrong ansswer 
/* impl Solution {
    pub fn check_valid_cuts(_n: i32, rectangles: Vec<Vec<i32>>) -> bool {
        let l = rectangles.len();
        let mut horizontal = vec![[0; 2]; l];
        let mut vertical = vec![[0; 2]; l];

        for (i, rect) in rectangles.iter().enumerate() {
            horizontal[i][0] = rect[0];
            horizontal[i][1] = rect[2];
            vertical[i][0] = rect[1];
            vertical[i][1] = rect[3];
        }

        horizontal.sort_by_key(|a| a[0]);
        vertical.sort_by_key(|a| a[0]);

        Self::find_cuts(&horizontal) || Self::find_cuts(&vertical)
    }

    fn find_cuts(axis: &[[i32; 2]]) -> bool {
        let mut count = 0;
        let mut cut = 1;

        for curr in axis.iter() {
            if curr[0] >= cut {
                count += 1;
            }

            cut = cut.max(curr[1]);

            if count >= 2 {
                return true;
            }
        }

        false
    }
}




*/
