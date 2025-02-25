//3360. Stone Removal Game

impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut x = 10;
        let mut k = 0;

        while n >= x {
            n -= x;
            x -= 1;
            k += 1;
        }

        k % 2 == 1
    }
}
