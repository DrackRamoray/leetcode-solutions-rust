pub struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; 2]; n];

        dp[0][0] = 0;
        dp[0][1] = k;

        for i in 1..n {
            dp[i][0] = dp[i-1][1];
            dp[i][1] = dp[i-1][0] * (k - 1) + dp[i-1][1] * (k - 1);
        }

        dp[n - 1][0] + dp[n - 1][1]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::num_ways(3, 2), 6);
    assert_eq!(Solution::num_ways(1, 1), 1);
    assert_eq!(Solution::num_ways(7, 2), 42);
}
