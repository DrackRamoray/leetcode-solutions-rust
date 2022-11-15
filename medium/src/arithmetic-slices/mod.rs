pub struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0;n];

        for i in 2..n {
            if nums[i-1] * 2 == nums[i-2] + nums[i] {
                dp[i] = dp[i-1] + 1;
            }
        }

        dp.iter().sum::<i32>()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1,2,3,4]), 3);
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
}
