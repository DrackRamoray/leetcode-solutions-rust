pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let sum: i32 = nums.iter().sum();
        let n = nums.len();
        let mut f = nums.iter().enumerate().fold(0, |acc, (i, x)| acc + (i as i32) * x);
        let mut res = f;

        for i in 1..n {
            f = f + sum - (n as i32) * nums[n - i];
            res = res.max(f)
        }

        res
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::max_rotate_function(vec![4,3,2,6]), 26);
    assert_eq!(Solution::max_rotate_function(vec![100]), 0);
}
