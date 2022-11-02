struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] < target {
                lo = mid + 1;
            } else {
                ans = mid;
                hi = mid - 1;
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1,3,5,6], 7), 4);
    }
}
