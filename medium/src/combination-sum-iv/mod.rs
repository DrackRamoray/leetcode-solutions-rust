pub struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0;target as usize + 1];
        dp[0] = 1;

        for i in 1..=target as usize {
            for &num in nums.iter() {
                if num as usize <= i {
                    dp[i] += dp[i-num as usize];
                }
            }
        }

        dp[target as usize]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::combination_sum4(vec![1,2,3], 4), 7);
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}
