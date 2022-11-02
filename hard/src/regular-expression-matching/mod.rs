struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_bytes();
        let pp = p.as_bytes();
        let n = ss.len();
        let m = pp.len();

        let mut dp = vec![vec![false;m+1];n+1];
        dp[0][0] = true;

        for i in 0..=n {
            for j in 1..=m {
                if pp[j-1] == b'*' {
                    dp[i][j] = dp[i][j] || dp[i][j-2];

                    if Self::matched(ss, pp, i, j - 1) {
                        dp[i][j] = dp[i][j] || dp[i-1][j];
                    }
                } else {
                    if Self::matched(ss, pp, i, j) {
                        dp[i][j] = dp[i][j] || dp[i-1][j-1];
                    }
                }
            }
        }

        dp[n][m]
    }

    fn matched(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
        if i == 0 {
            return false;
        }

        if p[j-1] == b'.' {
            return true;
        }

        return s[i-1] == p[j-1];
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "a*".to_owned()), true);
        assert_eq!(Solution::is_match("ab".to_owned(), ".*".to_owned()), true);
    }
}
