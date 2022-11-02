pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut lo = 0;
        let mut hi = n - 1;
        let mut p = 0;

        while p <= hi && hi < n {
            while hi < n && nums[hi] == 2 {
                hi -= 1;
            }
            if nums[p] == 0 {
                nums.swap(p, lo);
                lo += 1;
                p += 1;
            } else if nums[p] == 2 && hi < n {
                nums.swap(p, hi);
                hi -= 1;
            } else {
                p += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut nums = vec![2,0,2,1,1,0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,0,1,1,2,2]);
        let mut nums = vec![2,0,1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0,1,2]);
    }
}
