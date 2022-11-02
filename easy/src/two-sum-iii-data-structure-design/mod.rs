use std::collections::HashMap;

pub struct TwoSum {
    cache: HashMap<i32, i32>,
}

impl TwoSum {

    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn add(&mut self, number: i32) {
        let counter = self.cache.entry(number).or_insert(0);
        *counter += 1;
    }

    fn find(&self, value: i32) -> bool {
        for num in self.cache.keys() {
            if let Some(v) = self.cache.get(&(value - num)) {
                if *num != value - num || v > &1 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::TwoSum;

    #[test]
    fn it_works() {
        let mut two_some = TwoSum::new();
        two_some.add(1);
        two_some.add(3);
        two_some.add(5);
        let value = 4;
        let ans = true;
        assert_eq!(two_some.find(value), ans);
        let value = 7;
        let ans = false;
        assert_eq!(two_some.find(value), ans);
    }
}
