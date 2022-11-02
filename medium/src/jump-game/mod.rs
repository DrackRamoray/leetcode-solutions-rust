struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut distance = 0;

        for i in 0..n {
            if i <= distance {
                distance = distance.max(i + nums[i] as usize);

                if distance >= n - 1 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::can_jump(vec![2,3,1,1,4]), true);
        assert_eq!(super::Solution::can_jump(vec![3,2,1,0,4]), false);
    }
}
