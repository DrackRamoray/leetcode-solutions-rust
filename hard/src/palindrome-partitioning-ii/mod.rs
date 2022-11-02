pub struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![true;n];n];

        for i in (0..n).rev() {
            for j in i+1..n {
                dp[i][j] = (s[i..=i] == s[j..=j]) && dp[i+1][j-1];
            }
        }

        let mut ans = vec![n;n];

        for i in 0..n {
            if dp[0][i] {
                ans[i] = 0;
            } else {
                for j in 0..i {
                    if dp[j+1][i] && ans[j] + 1 < ans[i] {
                        ans[i] = ans[j] + 1;
                    }
                }
            }
        }

        ans[n-1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_cut("aab".into()), 1);
        assert_eq!(Solution::min_cut("a".into()), 0);
        assert_eq!(Solution::min_cut("ab".into()), 1);
    }
}
