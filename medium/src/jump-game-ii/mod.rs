pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0;n];

        for i in 0..n-1 {
            let num = nums[i] as usize;
            for j in 1..=num {
                if i + j >= n {
                    break;
                }

                dp[i+j] = if dp[i+j] == 0 {
                    dp[i] + 1
                } else {
                    dp[i+j].min(dp[i] + 1)
                };
            }
        }

        dp[n-1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::jump(vec![2,3,1,1,4]), 2);
        assert_eq!(Solution::jump(vec![2,3,0,1,4]), 2);
    }
}
