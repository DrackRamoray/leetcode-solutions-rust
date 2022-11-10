use std::cmp::Ordering::*;

pub struct Solution;

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n < 2 {
            return n as i32;
        }

        let mut up = vec![1;n];
        let mut down = vec![1;n];

        for i in 1..n {
            match nums[i].cmp(&nums[i-1]) {
                Greater => {
                    up[i] = up[i-1].max(down[i-1] + 1);
                    down[i] = down[i-1];
                },
                Less => {
                    up[i] = up[i-1];
                    down[i] = down[i-1].max(up[i-1] + 1);
                },
                Equal => {
                    up[i] = up[i-1];
                    down[i] = down[i-1];
                }
            }
        }

        down[n-1].max(up[n-1])
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::wiggle_max_length(vec![1,7,4,9,2,5]), 6);
    assert_eq!(Solution::wiggle_max_length(vec![1,17,5,10,13,15,10,5,16,8]), 7);
    assert_eq!(Solution::wiggle_max_length(vec![1,2,3,4,5,6,7,8,9]), 2)
}
