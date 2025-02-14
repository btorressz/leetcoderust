//1352. Product of the Last K Numbers

struct ProductOfNumbers {
  prefix_products: Vec<i32>,

}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {

     fn new() -> Self {
        Self {
            prefix_products: vec![1], // Initialize with 1 for easier calculations
        }
    }
    
  fn add(&mut self, num: i32) {
        if num == 0 {
            // Reset the list if a zero is added
            self.prefix_products = vec![1];
        } else {
            // Multiply the last element with the new number and append
            let last = *self.prefix_products.last().unwrap();
            self.prefix_products.push(last * num);
        }
    }
    
    fn get_product(&self, k: i32) -> i32 {
        let len = self.prefix_products.len();
        if len <= k as usize {
            0
        } else {
            self.prefix_products[len - 1] / self.prefix_products[len - k as usize - 1]
        }
    }
}


/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
