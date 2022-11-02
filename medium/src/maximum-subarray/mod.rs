struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MIN;
        let mut prev_sum = 0;

        for num in nums.into_iter() {
            prev_sum = num.max(prev_sum + num);
            ans = ans.max(prev_sum);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::max_sub_array(vec![-2,1,-3,4,-1,2,1,-5,4]), 6);
        assert_eq!(super::Solution::max_sub_array(vec![1]), 1);
        assert_eq!(super::Solution::max_sub_array(vec![5,4,-1,7,8]), 23);
    }
}
