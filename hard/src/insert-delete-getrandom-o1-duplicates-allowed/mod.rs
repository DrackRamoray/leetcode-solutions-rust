use rand::Rng;
use std::collections::{HashMap, HashSet};

pub struct RandomizedCollection {
    nums: Vec<i32>,
    indices: HashMap<i32, HashSet<usize>>,
}

impl RandomizedCollection {

    fn new() -> Self {
        Self {
            nums: vec![],
            indices: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        let entry = self.indices.entry(val).or_insert(HashSet::new());
        entry.insert(self.nums.len() - 1);

        entry.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(s) = self.indices.get_mut(&val) {
            if let Some(&index) = s.iter().next() {
                s.remove(&index);
                let last_index = self.nums.len() - 1;
                let last_val = self.nums[last_index];
                self.nums[index] = last_val;
                let entry = self.indices.entry(last_val).or_insert(HashSet::new());
                entry.insert(index);
                entry.remove(&last_index);
                self.nums.pop();

                true
            } else {
                false
            }
        } else {
            false
        }
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
    let mut obj = RandomizedCollection::new();
    assert_eq!(obj.remove(0), false);
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.insert(2), true);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.insert(2), false);
    assert_eq!(obj.remove(1), true);
    assert_eq!(obj.remove(1), false);
    assert_eq!(obj.remove(2), true);
    assert_eq!(obj.insert(1), true);
    assert_eq!(obj.remove(2), true);
}
