pub struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let n = nums.len();
        let mut nums1 = nums.clone();

        nums1.sort();

        let mut i = (n - 1) / 2;
        let mut j = n - 1;

        for k in 0..n {
            if k % 2 == 0 {
                nums[k] = nums1[i];
                i = i.wrapping_sub(1); // i -= 1;
            } else {
                nums[k] = nums1[j];
                j = j.wrapping_sub(1); // j -= 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut nums = vec![1,5,1,1,6,4];
    let ans = vec![1,6,1,5,1,4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, ans);

    let mut nums = vec![1,3,2,2,3,1];
    let ans = vec![2,3,1,3,1,2];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, ans);
}
