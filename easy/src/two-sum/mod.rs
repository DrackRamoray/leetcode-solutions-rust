struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = std::collections::HashMap::new();

        for i in 0..nums.len() {
            if let Some(&j) = m.get(&(target - nums[i])) {
                return vec![j as i32, i as i32];
            }

            m.insert(nums[i], i);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0, 1]);
    }
}
