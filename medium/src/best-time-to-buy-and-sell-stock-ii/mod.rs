pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0;2];n];
        dp[0][1] = -prices[0];

        for i in 1..n {
            dp[i][0] = dp[i-1][0].max(dp[i-1][1] + prices[i]);
            dp[i][1] = (dp[i-1][0] - prices[i]).max(dp[i-1][1]);
        }

        dp[n-1][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 7);
        assert_eq!(Solution::max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(Solution::max_profit(vec![7,6,4,3,1]), 0);
    }
}
