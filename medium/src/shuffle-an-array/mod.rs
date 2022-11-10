use rand::prelude::SliceRandom;
use rand::thread_rng;

#[allow(dead_code)]
pub struct Solution {
    nums: Vec<i32>,
}

impl Solution {
    #[allow(dead_code)]
    fn new(nums: Vec<i32>) -> Self {
        Solution { nums }
    }

    #[allow(dead_code)]
    fn reset(&self) -> Vec<i32> {
        self.nums.clone()
    }

    #[allow(dead_code)]
    fn shuffle(&self) -> Vec<i32> {
        let mut ans = self.nums.clone();
        ans.shuffle(&mut thread_rng());
        ans
    }
}
