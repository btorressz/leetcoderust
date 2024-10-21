impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        fn find_bit(n: i32, k: i32) -> char {
            if n == 1 {
                return '0';
            }
            
            let len = (1 << n) - 1;
            let mid = len / 2 + 1; 
        
            if k == mid {
                return '1';
            } else if k < mid {
                return find_bit(n - 1, k);
            } else {
                let mirrored_k = len - k + 1;
                let bit = find_bit(n - 1, mirrored_k);
                return if bit == '0' { '1' } else { '0' };
            }
        }

        find_bit(n, k)
    }
}
