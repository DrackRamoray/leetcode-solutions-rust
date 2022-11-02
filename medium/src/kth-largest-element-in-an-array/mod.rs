use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut h = BinaryHeap::with_capacity(k as usize);

        nums.into_iter().for_each(|x| {
            h.push(Reverse(x));

            if h.len() > k as usize {
                h.pop();
            }
        });

        if let Some(Reverse(v)) = h.pop() {
            v
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,1,5,6,4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4);
    }
}
