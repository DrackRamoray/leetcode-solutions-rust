struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if target == nums[mid] {
                return mid as i32;
            }

            if nums[0] <= nums[mid] {
                if nums[0] <= target && target < nums[mid] {
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[n-1] {
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 0), 4);
        assert_eq!(Solution::search(vec![4,5,6,7,0,1,2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
    }
}
