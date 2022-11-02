struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] > nums[mid+1] {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_peak_element(vec![1,2,3,1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1,2,1,3,5,6,4]), 5);
    }
}
