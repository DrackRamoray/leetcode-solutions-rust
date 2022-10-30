pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] < nums[hi] {
                hi = mid;
            } else if nums[mid] > nums[hi] {
                lo = mid + 1;
            } else {
                hi -= 1;
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
        assert_eq!(Solution::find_min(vec![1,3,5]), 1);
        assert_eq!(Solution::find_min(vec![2,2,2,0,1]), 0);
    }
}
