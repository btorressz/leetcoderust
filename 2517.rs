//2517. Maximum Tastiness of Candy Basket

impl Solution {
    pub fn maximum_tastiness(price: Vec<i32>, k: i32) -> i32 {
        // Sort the price array
        let mut price = price;
        price.sort();
        
        // Helper function to check if a given difference is possible
        fn is_accepted(price: &Vec<i32>, diff: i32, k: i32) -> bool {
            let mut count = 1;
            let mut last_price = price[0];
            let mut i = 0;
            
            while i < price.len() {
                if price[i] >= last_price + diff {
                    count += 1;
                    last_price = price[i];
                }
                i += 1;
            }
            count >= k
        }
        
        // Binary search for the max difference
        let (mut l, mut r) = (0, price[price.len() - 1] - price[0]);
        while l < r {
            let mid = (l + r + 1) / 2;
            if is_accepted(&price, mid, k) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        
        l
    }
}
