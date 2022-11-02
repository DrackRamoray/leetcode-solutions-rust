pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut mask = 0;

        for num in nums.iter() {
            mask = mask ^ num;
        }

        let diff = mask & (-mask);

        let mut found = 0;

        for num in nums.iter() {
            if num & diff != 0 {
                found = found ^ num;
            }
        }

        vec![found, mask ^ found]
    }
}

#[test]
fn it_works() {
    let nums = vec![1, 2, 1, 3, 2, 5];
    let ans = vec![3, 5];
    assert_eq!(Solution::single_number(nums), ans);
}
