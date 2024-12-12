impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        
        let mut n = n;
        while n > 0 {
            let d = a + b + c;
            a = b;
            b = c;
            c = d;
            n -= 1;
        }
        
        a
    }
}
