pub struct Solution;

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut coins = vec![1;n+2];
        coins[1..=n].clone_from_slice(&nums);
        let mut dp = vec![vec![0;n+2];n+2];

        for i in (0..=n).rev() {
            for j in i+1..=n+1 {
                for k in i+1..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + coins[i] * coins[k] * coins[j] + dp[k][j]);
                }
            }
        }

        dp[0][n+1]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::max_coins(vec![3,1,5,8]), 167);
    assert_eq!(Solution::max_coins(vec![1,5]), 10);
}
