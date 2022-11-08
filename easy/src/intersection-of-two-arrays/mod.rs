use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<HashSet<i32>>()
            .intersection(&nums2
                .into_iter()
                .collect::<HashSet<i32>>()
            )
            .cloned()
            .collect()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::intersection(vec![1,2,2,1], vec![2,2]), vec![2]);
}
