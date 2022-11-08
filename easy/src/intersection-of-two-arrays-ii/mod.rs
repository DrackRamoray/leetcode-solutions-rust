pub struct Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort();
        nums2.sort();

        let mut ans = vec![];
        let mut i = 0;
        let mut j = 0;

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                ans.push(nums1[i]);
                i += 1;
                j += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else {
                i += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::intersect(vec![1,2,2,1], vec![2,2,]), vec![2,2]);
    assert_eq!(Solution::intersect(vec![4,9,5], vec![9,4,9,8,4]), vec![4,9]);
}
