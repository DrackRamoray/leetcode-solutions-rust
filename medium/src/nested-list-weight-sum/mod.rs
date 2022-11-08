#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

pub struct Solution;

impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        Self::weights(&nested_list, 1)
    }

    fn weights(nested_list: &Vec<NestedInteger>, depth: i32) -> i32 {
        let mut sum = 0;

        for list in nested_list {
            match list {
                NestedInteger::Int(v) => sum += depth * v,
                NestedInteger::List(nested) => sum += Self::weights(nested, depth + 1)
            }
        }

        sum
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::depth_sum(vec![NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]), NestedInteger::Int(2), NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])]), 10);
    assert_eq!(Solution::depth_sum(vec![NestedInteger::Int(1), NestedInteger::List(vec![NestedInteger::Int(4), NestedInteger::List(vec![NestedInteger::Int(6)])])]), 27);
    assert_eq!(Solution::depth_sum(vec![NestedInteger::Int(0)]), 0);
}
