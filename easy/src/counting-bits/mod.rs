pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut dp = vec![0;n + 1];

        for i in 1..=n {
            dp[i] = dp[i & (i-1)] + 1
        }

        dp
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_bits(2), vec![0,1,1]);
    assert_eq!(Solution::count_bits(5), vec![0,1,1,2,1,2]);
}
