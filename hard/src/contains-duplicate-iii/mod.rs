pub struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, k: i32, t: i32) -> bool {
        let n = nums.len();

        for i in 0..n {
            let mut j = i + 1;

            while j <= i + k as usize && j < n {
                if (nums[j] as i64 - nums[i] as i64).abs() <= t as i64 {
                    return true;
                }
                j += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,2,3,1], 3, 0), true);
        assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,0,1,1], 1, 2), true);
        assert_eq!(Solution::contains_nearby_almost_duplicate(vec![1,5,9,1,5,9], 2, 3), false);
    }
}
