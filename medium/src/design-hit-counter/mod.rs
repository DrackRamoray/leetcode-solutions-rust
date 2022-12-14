use std::collections::VecDeque;

pub struct HitCounter {
    hits: VecDeque<i32>,
}

impl HitCounter {

    fn new() -> Self {
        Self {
            hits: VecDeque::new(),
        }
    }

    fn hit(&mut self, timestamp: i32) {
        while let Some(t) = self.hits.front() {
            if t + 300 <= timestamp {
                self.hits.pop_front();
            } else {
                break;
            }
        }

        self.hits.push_back(timestamp);
    }

    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(t) = self.hits.front() {
            if t + 300 <= timestamp {
                self.hits.pop_front();
            } else {
                break;
            }
        }

        self.hits.len() as i32
    }
}


#[test]
fn it_works() {
    let mut obj = HitCounter::new();
    obj.hit(1);
    obj.hit(2);
    obj.hit(3);
    assert_eq!(obj.get_hits(4), 3);
    obj.hit(300);
    assert_eq!(obj.get_hits(300), 4);
    assert_eq!(obj.get_hits(301), 3);
}
