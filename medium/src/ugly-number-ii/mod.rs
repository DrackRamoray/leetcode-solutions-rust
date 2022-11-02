pub struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1];
        let mut i = 1;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        while i < n {
            dp.push((dp[i2] * 2).min(dp[i3] * 3).min(dp[i5] * 5));

            while dp[i2] * 2 <= dp[i] {
                i2 += 1;
            }

            while dp[i3] * 3 <= dp[i] {
                i3 += 1;
            }

            while dp[i5] * 5 <= dp[i] {
                i5 += 1;
            }

            i += 1;
        }

        dp[n-1]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
    assert_eq!(Solution::nth_ugly_number(1), 1);
}
