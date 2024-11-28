//599. Minimum Index Sum of Two Lists
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut min_sum = usize::MAX; 
        let mut result = Vec::new(); 

        for i in 0..list1.len() {
            for j in 0..list2.len() {
                if list1[i] == list2[j] { 
                    let index_sum = i + j;

                    if index_sum < min_sum {
                        min_sum = index_sum;
                        result = vec![list1[i].clone()];
                    } 
                    else if index_sum == min_sum {
                        result.push(list1[i].clone());
                    }
                }
            }
        }

        result
    }
}
