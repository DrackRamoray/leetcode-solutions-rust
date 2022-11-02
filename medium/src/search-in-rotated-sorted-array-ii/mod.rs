pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[lo] < nums[mid] {
                if nums[lo] <= target && target <= nums[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            } else if nums[lo] > nums[mid] {
                if nums[lo] <= target || target <= nums[mid] {
                    hi = mid;
                } else {
                    lo = mid + 1;
                }
            } else if nums[lo] == nums[mid] {
                if nums[lo] != target {
                    lo += 1;
                } else {
                    hi = lo;
                }
            }
        }

        nums[lo] == target
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 0), true);
        assert_eq!(Solution::search(vec![2,5,6,0,0,1,2], 3), false);
    }
}
