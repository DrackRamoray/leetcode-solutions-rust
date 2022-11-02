struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left = Self::left_bound(&nums, target);

        if left == -1 {
            return vec![-1, -1];
        }

        let right = Self::right_bound(&nums, target);

        vec![left, right]
    }

    fn left_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] >= target {
                hi = mid - 1;
                ans = mid;
            } else {
                lo = mid + 1;
            }
        }

        if ans == n || nums[ans] != target {
            -1
        } else {
            ans as i32
        }
    }

    fn right_bound(nums: &Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut ans = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if nums[mid] <= target {
                lo = mid + 1;
                ans = mid;
            } else {
                hi = mid - 1;
            }
        }

        if ans == n || nums[ans] != target {
            -1
        } else {
            ans as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 8), vec![3,4]);
        assert_eq!(Solution::search_range(vec![5,7,7,8,8,10], 6), vec![-1,-1]);
        assert_eq!(Solution::search_range(vec![], 0), vec![-1,-1]);
    }
}
