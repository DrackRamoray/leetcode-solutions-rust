pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();

        if sum % 2 != 0 {
            return false;
        }

        let target = sum / 2;
        let &max_num = nums.iter().max().unwrap();

        if target < max_num {
            return false;
        }

        let n = nums.len();
        let target = target as usize;
        let mut dp = vec![vec![false;target + 1];n];

        for i in 0..n {
            dp[i][0] = true;
        }
        dp[0][nums[0] as usize] = true;

        for i in 1..n {
            let num = nums[i] as usize;

            for j in 1..=target {
                if j >= num {
                    dp[i][j] = dp[i-1][j] || dp[i-1][j-num];
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }

        dp[n-1][target]
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_partition(vec![1,5,11,5]), true);
    assert_eq!(Solution::can_partition(vec![1,2,3,5]), false);
}
