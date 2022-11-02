pub struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort();
        let mut i = 1;

        while i < nums.len() - 1 {
            nums.swap(i, i + 1);
            i += 2;
        }
    }
}

#[test]
fn it_works() {
    let mut nums = vec![3,5,2,1,6,4];
    let ans = vec![1,3,2,5,4,6];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, ans);

    let mut nums = vec![6,6,5,6,3,8];
    let ans = vec![3,6,5,6,6,8];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, ans);
}
