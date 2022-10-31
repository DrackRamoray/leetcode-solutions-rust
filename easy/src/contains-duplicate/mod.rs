pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s = std::collections::HashSet::new();

        for num in nums.iter() {
            if s.contains(num) {
                return true;
            }

            s.insert(num);
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
        assert_eq!(Solution::contains_duplicate(nums), true);
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::contains_duplicate(nums), false);
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(Solution::contains_duplicate(nums), true);
    }
}
