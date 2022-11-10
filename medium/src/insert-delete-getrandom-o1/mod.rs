use rand::Rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    nums: Vec<i32>,
    indices: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            nums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.indices.contains_key(&val) {
            return false;
        }

        let index = self.nums.len();
        self.nums.push(val);
        self.indices.insert(val, index);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.indices.get(&val) {
            let last_index = self.nums.len() - 1;
            self.nums[index] = self.nums[last_index];
            self.indices.insert(self.nums[last_index], index);
            self.indices.remove(&val);
            self.nums.pop();

            return true;
        }

        false
    }

    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..self.nums.len());
        self.nums[index]
    }
}

#[test]
fn it_works() {
    let mut obj = RandomizedSet::new();
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), false);
    assert_eq!(obj.insert(2), true);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.insert(2), false);
}
