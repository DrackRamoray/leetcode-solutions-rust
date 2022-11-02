struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 1 {
            return nums[0];
        }

        Self::do_rob(&nums, 0, n - 2).max(Self::do_rob(&nums, 1, n - 1))
    }

    fn do_rob(nums: &Vec<i32>, begin: usize, end: usize) -> i32 {
        if begin == end {
            return nums[begin];
        }

        let mut dp = vec![0;nums.len()];
        dp[begin] = nums[begin];
        dp[begin+1] = nums[begin].max(nums[begin+1]);

        for i in begin+2..=end {
            dp[i] = dp[i-1].max(dp[i-2] + nums[i]);
        }

        dp[end]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::rob(vec![2,3,2]), 3);
        assert_eq!(Solution::rob(vec![1,2,3,1]), 4);
        assert_eq!(Solution::rob(vec![1,2,3]), 3);
    }
}
