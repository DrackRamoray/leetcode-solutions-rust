pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut dp = vec![vec![0;3];n];
        dp[0][0] = -prices[0];

        for i in 1..n {
            dp[i][0] = dp[i-1][0].max(dp[i-1][2] - prices[i]); // 持有一支股票
            dp[i][1] = dp[i-1][0] + prices[i]; // 不持有股票，处于冷冻期
            dp[i][2] = dp[i-1][1].max(dp[i-1][2]); // 不持有股票，不处于冷冻期
        }

        dp[n-1][1].max(dp[n-1][2])
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::max_profit(vec![1,2,3,0,2]), 3);
    assert_eq!(Solution::max_profit(vec![1]), 0);
}
