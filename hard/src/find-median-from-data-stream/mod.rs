use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct MedianFinder {
    min_queue: BinaryHeap<i32>,
    max_queue: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        Self {
            min_queue: BinaryHeap::new(),
            max_queue: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match self.min_queue.peek() {
            Some(&v) => match v >= num {
                true => {
                    self.min_queue.push(num);

                    if self.min_queue.len() > self.max_queue.len() + 1 {
                        if let Some(u) = self.min_queue.pop() {
                            self.max_queue.push(Reverse(u));
                        }
                    }
                }
                false => {
                    self.max_queue.push(Reverse(num));

                    if self.max_queue.len() > self.min_queue.len() {
                        if let Some(Reverse(u)) = self.max_queue.pop() {
                            self.min_queue.push(u);
                        }
                    }
                }
            },
            None => {
                self.min_queue.push(num);
            }
        }
    }

    fn find_median(&self) -> f64 {
        if self.min_queue.len() > self.max_queue.len() {
            if let Some(&v) = self.min_queue.peek() {
                return v as f64;
            }
        }

        if let (Some(&v), Some(&Reverse(u))) = (self.min_queue.peek(), self.max_queue.peek()) {
            return (v as f64 + u as f64) / 2.0;
        }

        -1.0
    }
}

#[test]
fn it_works() {
    let mut obj = MedianFinder::new();
    obj.add_num(1);
    obj.add_num(2);
    assert_eq!(obj.find_median(), 1.5);
    obj.add_num(3);
    assert_eq!(obj.find_median(), 2.0);
}
