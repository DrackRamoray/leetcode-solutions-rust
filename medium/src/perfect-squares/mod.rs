pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0;n + 1];

        for i in 1..=n {
            let mut cnt = i32::MAX;
            let mut j = 1;

            while j * j <= i {
                cnt = cnt.min(dp[i - j * j]);
                j += 1;
            }

            dp[i] = cnt + 1;
        }

        dp[n]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::num_squares(12), 3);
    assert_eq!(Solution::num_squares(13), 2);
}
