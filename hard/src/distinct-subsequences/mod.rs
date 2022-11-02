pub struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        let n = ss.len();
        let m = tt.len();

        if n < m {
            return 0;
        }

        let mut dp = vec![vec![0;m+1];n+1];

        for i in 0..=n {
            dp[i][m] = 1;
        }

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if ss[i] == tt[j] {
                    dp[i][j] = dp[i+1][j+1] + dp[i+1][j];
                } else {
                    dp[i][j] = dp[i+1][j];
                }
            }
        }

        dp[0][0]
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_distinct("rabbbit".into(), "rabbit".into()), 3);
        assert_eq!(Solution::num_distinct("babgbag".into(), "bag".into()), 5);
    }
}
