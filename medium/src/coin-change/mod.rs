pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp = vec![i32::MAX-1;amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for &coin in coins.iter() {
                if i >= coin as usize {
                    dp[i] = dp[i].min(dp[i - coin as usize] + 1);
                }
            }
        }

        if dp[amount] == i32::MAX - 1 {
            -1
        } else {
            dp[amount]
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::coin_change(vec![1,2,5], 11), 3);
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}
