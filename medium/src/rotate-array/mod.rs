pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums[..].reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut nums = vec![1,2,3,4,5,6,7];
        Solution::rotate(&mut nums, 3);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);

        let mut nums = vec![-1,-100,3,99];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![3,99,-1,-100]);
    }
}
