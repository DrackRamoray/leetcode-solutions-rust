pub struct Solution;

impl Solution {
    pub fn strobogrammatic_in_range(low: String, high: String) -> i32 {
        let mut count = 0;

        Self::dfs("".into(), &high, &low, &mut count);
        Self::dfs("0".into(), &high, &low, &mut count);
        Self::dfs("1".into(), &high, &low, &mut count);
        Self::dfs("8".into(), &high, &low, &mut count);

        count
    }

    fn dfs(s: String, high: &str, low: &str, count: &mut i32) {
        let cur = s.len();
        let max_len = high.len();

        if cur > max_len {
            return;
        }

        if cur == max_len && (s.len() > 0 && s.parse::<i64>().unwrap() - high.parse::<i64>().unwrap() > 0) {
            return;
        }

        let b = (cur >= 2 && s.starts_with("0")) || (s.len() > 0 && (s.parse::<i64>().unwrap() - low.parse::<i64>().unwrap() < 0));

        if !b && cur > 0 {
            *count += 1;
        }

        Self::dfs(format!("{}{}{}", 0, s, 0), high, low, count);
        Self::dfs(format!("{}{}{}", 1, s, 1), high, low, count);
        Self::dfs(format!("{}{}{}", 6, s, 9), high, low, count);
        Self::dfs(format!("{}{}{}", 8, s, 8), high, low, count);
        Self::dfs(format!("{}{}{}", 9, s, 6), high, low, count);
    }
}

#[test]
fn it_works() {
    let low = "50".to_string();
    let high = "100".to_string();
    let ans = 3;
    assert_eq!(Solution::strobogrammatic_in_range(low, high), ans);
    let low = "0".to_string();
    let high = "0".to_string();
    let ans = 1;
    assert_eq!(Solution::strobogrammatic_in_range(low, high), ans);
}
