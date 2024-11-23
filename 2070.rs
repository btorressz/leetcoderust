impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut items = items;
        let mut queries = queries
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, i32)>>();

        items.sort_unstable_by_key(|item| item[0]);
        let mut max_beauty = 0;
        for item in &mut items {
            max_beauty = max_beauty.max(item[1]);
            item[1] = max_beauty;
        }

        queries.sort_unstable_by_key(|&(_, query)| query);

        let mut result = vec![0; queries.len()];
        let mut i = 0; 
        for (original_idx, query) in queries {
            while i < items.len() && items[i][0] <= query {
                i += 1;
            }
            result[original_idx] = if i > 0 { items[i - 1][1] } else { 0 };
        }

        result
    }
}
