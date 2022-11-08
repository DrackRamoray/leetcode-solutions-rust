#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct NestedIterator {
    index: usize,
    data: Vec<i32>,
}

impl NestedIterator {

    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut data = vec![];
        Self::flatten(&nested_list, &mut data);

        Self {
            index: 0,
            data,
        }
    }

    fn next(&mut self) -> i32 {
        let v = self.data[self.index];
        self.index += 1;
        v
    }

    fn has_next(&self) -> bool {
        self.index < self.data.len()
    }

    fn flatten(nested_list: &Vec<NestedInteger>, data: &mut Vec<i32>) {
        for item in nested_list.iter() {
            match item {
                NestedInteger::Int(v) => {
                    data.push(*v);
                },
                NestedInteger::List(inner_list) => {
                    Self::flatten(&inner_list, data);
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let nested_list = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 1, 2, 1, 1];
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);

    let nested_list = vec![
        NestedInteger::Int(1),
        NestedInteger::List(vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![NestedInteger::Int(6)]),
        ]),
    ];
    let mut obj = NestedIterator::new(nested_list);
    let res = vec![1, 4, 6];
    let mut ans = vec![];
    while obj.has_next() {
        ans.push(obj.next());
    }
    assert_eq!(ans, res);
}
