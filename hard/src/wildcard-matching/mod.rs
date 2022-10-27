pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_bytes();
        let pp = p.as_bytes();
        let n = ss.len();
        let m = pp.len();

        let mut dp = vec![vec![false;m+1];n+1];
        dp[0][0] = true;

        for j in 1..=m {
            if pp[j-1] != b'*' {
                break;
            }

            dp[0][j] = true;
        }

        for i in 1..=n {
            for j in 1..=m {
                if pp[j-1] == b'*' {
                    dp[i][j] = dp[i-1][j] || dp[i][j-1];
                } else if pp[j-1] == b'?' || ss[i-1] == pp[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_match("aa".to_owned(), "a".to_owned()), false);
        assert_eq!(Solution::is_match("aa".to_owned(), "*".to_owned()), true);
        assert_eq!(Solution::is_match("cb".to_owned(), "?a".to_owned()), false);
        assert_eq!(Solution::is_match("adceb".to_owned(), "*a*b".to_owned()), true);
        assert_eq!(Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned()), false);
    }
}
