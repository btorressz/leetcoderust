
//attempt four successful

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let n = n as i64;
        let m = m as i64;

        let half_up_n = (n + 1) / 2;     // Odd x count
        let half_down_n = n / 2;         // Even x count
        let half_up_m = (m + 1) / 2;     // Odd y count
        let half_down_m = m / 2;         // Even y count

        // Only (odd, even) or (even, odd) pairs make x + y odd
        half_up_n * half_down_m + half_down_n * half_up_m
    }
}


//attempt three wrong answer 

/*

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let half_up_n = (n + 1) / 2;     // Odd x count
        let half_down_n = n / 2;         // Even x count
        let half_up_m = (m + 1) / 2;     // Odd y count
        let half_down_m = m / 2;         // Even y count

        // Only (odd, even) or (even, odd) pairs make x + y odd
        let res = (half_up_n * half_down_m + half_down_n * half_up_m) as i64;
        res
    }
}


*/


//attempt two wrong answer

/*

impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let half_up_n = (n + 1) / 2;     // odd x count
        let half_up_m = m / 2;           // even y count
        let half_down_n = n / 2;         // even x count
        let half_down_m = (m + 1) / 2;   // odd y count

        let res = (half_up_n * half_down_m + half_down_n * half_up_m) as i64;
        res
    }
}



*/

//attempt one wrong answer 
/*impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let half_up_n = (n + 1) / 2;
        let half_up_m = (m + 1) / 2;
        let half_down_n = n / 2;
        let half_down_m = m / 2;

        let res = (half_up_n * half_down_m + half_down_n * half_up_m) as i64;
        res
    }
}
*/
