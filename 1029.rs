//1029. Two City Scheduling

impl Solution {
    pub fn two_city_sched_cost(mut costs: Vec<Vec<i32>>) -> i32 {
        costs.sort_by_key(|cost| cost[0] - cost[1]);

        let n = costs.len() / 2;
        let mut res = 0;

        for i in 0..n {
            res += costs[i][0] + costs[i + n][1];
        }

        res
    }
}
