pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let mut dp = vec![vec![0;m+1];n+1];

        for i in 1..=n {
            dp[i][0] = i;
        }

        for j in 1..=m {
            dp[0][j] = j;
        }

        for i in 1..=n {
            for j in 1..=m {
                if w1[i-1] == w2[j-1] {
                    dp[i][j] = dp[i-1][j-1];
                } else {
                    dp[i][j] = dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]) + 1;
                }
            }
        }

        dp[n][m] as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(super::Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}
