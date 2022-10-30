pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp_max = vec![nums[0];n];
        let mut dp_min = vec![nums[0];n];

        for i in 1..n {
            dp_max[i] = nums[i].max(dp_max[i-1] * nums[i]).max(dp_min[i-1] * nums[i]);
            dp_min[i] = nums[i].min(dp_min[i-1] * nums[i]).min(dp_max[i-1] * nums[i]);
        }

        dp_max.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_product(vec![2,3,-2,4]), 6);
        assert_eq!(Solution::max_product(vec![-2,0,-1]), 0);
    }
}
