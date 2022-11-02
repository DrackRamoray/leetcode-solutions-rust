struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }

        let n = s1.len();
        let m = s2.len();
        let mut dp = vec![vec![false;m + 1]; n + 1];

        dp[0][0] = true;

        for i in 0..=n {
            for j in 0..=m {
                let k = i.wrapping_add(j).wrapping_sub(1); // i + j - 1

                if i > 0 {
                    dp[i][j] = dp[i][j] || (dp[i-1][j] && s1[i-1..=i-1] == s3[k..=k]);
                }

                if j > 0 {
                    dp[i][j] = dp[i][j] || (dp[i][j-1] && s2[j-1..=j-1] == s3[k..=k]);
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
        assert_eq!(Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbcbcac".into()), true);
        assert_eq!(Solution::is_interleave("aabcc".into(), "dbbca".into(), "aadbbbaccc".into()), false);
        assert_eq!(Solution::is_interleave("".into(), "".into(), "".into()), true);
    }
}
