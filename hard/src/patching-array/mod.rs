pub struct Solution;

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let m = nums.len();
        let mut ans = 0;
        let mut miss = 1;
        let mut i = 0;

        while miss <= n as i64 {
            if i < m && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss *= 2;
                ans += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_patches(vec![1,3], 6), 1);
    assert_eq!(Solution::min_patches(vec![1,5,10], 20), 2);
    assert_eq!(Solution::min_patches(vec![1,2,2], 5), 0);
}
