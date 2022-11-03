pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut lo = 0;
        let mut hi = 0;

        while hi < nums.len() {
            if nums[hi] != 0 {
                nums.swap(lo, hi);
                lo += 1;
            }

            hi += 1;
        }
    }
}

#[test]
fn it_works() {
    let mut nums = vec![0,1,0,3,12];
    let ans = vec![1,3,12,0,0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, ans);

    let mut nums = vec![0];
    let ans = vec![0];
    Solution::move_zeroes(&mut nums);
    assert_eq!(nums, ans);
}
