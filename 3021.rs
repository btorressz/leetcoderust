//3021 Alice and Bob Playing Flower Game

//In my first two attempts, I mistakenly did the multiplication using i32, which caused integer overflow for large inputs even though I cast the final result to i64. The cast came too late 
//after the overflow had already happened.
//In the third attempt, I also mixed up variables, using n where I should’ve used m, introducing another bug.
//In the fourth attempt, I fixed both issues by casting n and m to i64 before doing any math and corrected the variable usage. That made the solution accurate and passed all test cases.

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
