//2437. Number of Valid Clock Times

impl Solution {
    pub fn count_time(time: String) -> i32 {
        let mut res = 1;
        let time: Vec<char> = time.chars().collect();

        if time[3] == '?' {
            res *= 6;
        }
        if time[4] == '?' {
            res *= 10;
        }
        if time[0] == '?' && time[1] == '?' {
            res *= 24;
        } else {
            if time[0] == '?' {
                if time[1] > '3' {
                    res *= 2;
                } else {
                    res *= 3;
                }
            }
            if time[1] == '?' {
                if time[0] == '2' {
                    res *= 4;
                } else {
                    res *= 10;
                }
            }
        }

        res
    }
}
