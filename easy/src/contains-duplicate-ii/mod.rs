struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let n = nums.len();
        let mut m = HashMap::new();

        for i in 0..n {
            if let Some(&j) = m.get(&nums[i]) {
                if (i as i32 - j as i32).abs() <= k {
                    return true;
                }
            }

            m.insert(nums[i], i);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 3), true);
        let nums = vec![1, 0, 1, 1];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 1), true);
        let nums = vec![1, 2, 3, 1, 2, 3];
        assert_eq!(Solution::contains_nearby_duplicate(nums, 2), false);
    }
}
