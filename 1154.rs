//1154. Day of the Year

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let parts: Vec<i32> = date
            .split('-')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let (y, m, d) = (parts[0], parts[1], parts[2]);

        let leap_year = y % 400 == 0 || (y % 4 == 0 && y % 100 != 0);
        let feb_days = if leap_year { 29 } else { 28 };

        let days_in_month = vec![31, feb_days, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        days_in_month.iter().take((m - 1) as usize).sum::<i32>() + d
    }
}
