use assist::ListNode;
use rand::{thread_rng, Rng};

#[allow(dead_code)]
pub struct Solution {
    data: Vec<i32>,
}

impl Solution {
    #[allow(dead_code)]
    fn new(mut head: Option<Box<ListNode>>) -> Self {
        let mut data = vec![];

        while let Some(mut h) = head {
            data.push(h.val);
            head = h.next.take();
        }

        Self {
            data,
        }
    }

    #[allow(dead_code)]
    fn get_random(&self) -> i32 {
        self.data[thread_rng().gen_range(0..self.data.len())]
    }
}
