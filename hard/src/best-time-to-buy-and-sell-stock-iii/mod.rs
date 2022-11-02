struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![vec![0;3];2];n];
        dp[0][0][0] = 0;
        dp[0][0][1] = -100000;
        dp[0][0][2] = -100000;
        dp[0][1][0] = -prices[0];
        dp[0][1][1] = -100000;
        dp[0][1][2] = -100000;

        for i in 1..n {
            dp[i][0][0] = 0;
            dp[i][0][1] = dp[i-1][0][1].max(dp[i-1][1][0] + prices[i]);
            dp[i][0][2] = dp[i-1][0][2].max(dp[i-1][1][1] + prices[i]);
            dp[i][1][0] = dp[i-1][1][0].max(dp[i-1][0][0] - prices[i]);
            dp[i][1][1] = dp[i-1][1][1].max(dp[i-1][0][1] - prices[i]);
            dp[i][1][2] = -100000;
        }

        0.max(dp[n-1][0][1]).max(dp[n-1][0][2])
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![3,3,5,0,0,3,1,4]), 6);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(Solution::max_profit(vec![1]), 0);
    }
}
