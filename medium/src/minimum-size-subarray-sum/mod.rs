struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n == 0 {
            return 0;
        }

        let mut ans = n + 1;
        let mut left = 0;
        let mut sum = 0;

        for right in 0..n {
            sum += nums[right];

            while sum >= target && left <= right {
                ans = ans.min(right - left + 1);
                sum -= nums[left];
                left += 1;
            }
        }

        if ans == n + 1 {
            return 0;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2,3,1,2,4,3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1,4,4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1,1,1,1,1,1,1,1]), 0);
    }
}
