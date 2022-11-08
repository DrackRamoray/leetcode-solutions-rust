use std::collections::{BinaryHeap, HashMap};

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();

        if n == 0 {
            return Vec::new();
        }

        let mut m = HashMap::new();

        nums.into_iter().for_each(|x| {
            let k = m.entry(x).or_insert(0);
            *k -= 1;
        });

        let mut h = BinaryHeap::with_capacity(k as usize);

        m.into_iter().for_each(|x| {
            h.push((x.1, x.0));

            if h.len() > k as usize {
                h.pop();
            }
        });

        h.into_iter().map(|x| x.1).collect::<Vec<i32>>()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::top_k_frequent(vec![1,1,1,2,2,3], 2), vec![2,1]);
    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
}
