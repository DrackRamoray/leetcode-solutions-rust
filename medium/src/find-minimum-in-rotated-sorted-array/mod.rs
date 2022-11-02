struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] < nums[hi] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        nums[lo]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_min(vec![3,4,5,1,2]), 1);
        assert_eq!(Solution::find_min(vec![4,5,6,7,0,1,2]), 0);
        assert_eq!(Solution::find_min(vec![11,13,15,17]), 11);
    }
}
