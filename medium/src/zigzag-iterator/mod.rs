use std::collections::VecDeque;

pub struct ZigzagIterator {
    data: VecDeque<i32>
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut data = VecDeque::new();
        let n = v1.len();
        let m = v2.len();
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            data.push_back(v1[i]);
            data.push_back(v2[j]);
            i += 1;
            j += 1;
        }

        while i < n {
            data.push_back(v1[i]);
            i += 1;
        }

        while j < m {
            data.push_back(v2[j]);
            j += 1;
        }

        Self {
            data,
        }
    }

    fn next(&mut self) -> i32 {
        if let Some(v) = self.data.pop_front() {
            v
        } else {
            -1
        }
    }

    fn has_next(&self) -> bool {
        self.data.len() > 0
    }
}

#[test]
fn it_works() {
    let v1 = vec![1, 2];
    let v2 = vec![3, 4, 5, 6];
    let res = vec![1, 3, 2, 4, 5, 6];
    let mut obj = ZigzagIterator::new(v1, v2);
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);
}
