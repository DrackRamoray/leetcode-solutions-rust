use std::collections::HashMap;
use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub struct Solution {
    data: HashMap<i32, Vec<usize>>
}

impl Solution {
    #[allow(dead_code)]
    fn new(nums: Vec<i32>) -> Self {
        let mut data = HashMap::new();

        for i in 0..nums.len() {
            data.entry(nums[i]).or_insert(vec![]).push(i);
        }

        Self {
            data,
        }
    }

    #[allow(dead_code)]
    fn pick(&self, target: i32) -> i32 {
        if let Some(vals) = self.data.get(&target) {
            let mut rng = thread_rng();
            vals[rng.gen_range(0..vals.len())] as i32
        } else {
            -1
        }
    }
}
