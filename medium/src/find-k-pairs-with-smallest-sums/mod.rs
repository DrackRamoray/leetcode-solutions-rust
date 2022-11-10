use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let k = k as usize;

        for i in 0..(k.min(nums1.len())) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        let mut ans = vec![];

        for _ in (1..=k).rev() {
            if let Some(Reverse((_, j, k))) = heap.pop() {
                ans.push(vec![nums1[j], nums2[k]]);

                if k < nums2.len() - 1 {
                    heap.push(Reverse((nums1[j] + nums2[k+1], j, k + 1)));
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::k_smallest_pairs(vec![1,7,11], vec![2,4,6], 3), vec![vec![1,2],vec![1,4],vec![1,6]]);
    assert_eq!(Solution::k_smallest_pairs(vec![1,1,2], vec![1,2,3], 2), vec![vec![1,1],vec![1,1]]);
    assert_eq!(Solution::k_smallest_pairs(vec![1,2], vec![3], 3), vec![vec![1,3],vec![2,3]]);
}
