use std::collections::HashSet;

pub struct PhoneDirectory {
    nums: HashSet<i32>,
}

impl PhoneDirectory {

    fn new(max_numbers: i32) -> Self {
        Self {
            nums: (0..max_numbers).collect::<HashSet<i32>>(),
        }
    }

    fn get(&mut self) -> i32 {
        if let Some(&num) = self.nums.iter().next() {
            self.nums.remove(&num);
            num
        } else {
            -1
        }
    }

    fn check(&self, number: i32) -> bool {
        if self.nums.contains(&number) {
            return true;
        }

        false
    }

    fn release(&mut self, number: i32) {
        self.nums.insert(number);
    }
}

#[test]
fn it_works() {
    let mut obj = PhoneDirectory::new(1);
    assert_eq!(obj.get(), 0);
    assert_eq!(obj.check(0), false);
    assert_eq!(obj.get(), -1);
    obj.release(2);
    assert_eq!(obj.check(2), true);
}
