use std::collections::VecDeque;

pub struct MovingAverage {
    size: usize,
    data: VecDeque<i32>,
}

impl MovingAverage {

    fn new(size: i32) -> Self {
        Self {
            size: size as usize,
            data: VecDeque::with_capacity(size as usize),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.data.len() >= self.size {
            self.data.pop_front();
        }

        self.data.push_back(val);

        self.data.iter().map(|&x| x as f64).sum::<f64>() / (self.data.len() as f64)
    }
}

#[test]
fn it_works() {
    let mut m = MovingAverage::new(3);

    assert_eq!(m.next(1), 1f64);
    assert_eq!(m.next(10), 5.5f64);
    assert_eq!(m.next(3), 14f64 / 3f64);
    assert_eq!(m.next(5), 6f64);
}
