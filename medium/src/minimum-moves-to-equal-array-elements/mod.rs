pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();
        let mut ans = 0;

        for num in nums.iter() {
            ans += num - min;
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_moves(vec![1,2,3]), 3);
    assert_eq!(Solution::min_moves(vec![1,1,1]), 0);
}
