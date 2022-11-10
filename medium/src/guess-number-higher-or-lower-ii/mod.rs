pub struct Solution;

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0;n+1];n+1];

        for i in 1..n {
            for j in 1..n-i+1 {
                dp[j][i+j] = (i/2+j..i+j).map(|v| v as i32 + dp[j][v-1].max(dp[v+1][i+j])).min().unwrap_or(0);
            }
        }

        dp[1][n]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::get_money_amount(10), 16);
    assert_eq!(Solution::get_money_amount(1), 0);
    assert_eq!(Solution::get_money_amount(2), 1);
}
