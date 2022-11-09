#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

pub struct Solution;

impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        let mut vec = vec![];
        let mut max_depth = 1;
        Self::depth_sum(&nested_list, 1, &mut max_depth, &mut vec);

        vec.iter().fold(0, |acc, (d, v)| acc + (max_depth - d + 1) * v)
    }

    fn depth_sum(nested_list: &Vec<NestedInteger>, depth: i32, max_depth: &mut i32, vec: &mut Vec<(i32, i32)>) {
        for list in nested_list {
            *max_depth = (*max_depth).max(depth);
            match list {
                NestedInteger::Int(v) => {
                    vec.push((depth, *v));
                },
                NestedInteger::List(sub) => Self::depth_sum(sub, depth + 1, max_depth, vec)
            }
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::depth_sum_inverse(vec![NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]), NestedInteger::Int(2), NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)])]), 8);
    assert_eq!(Solution::depth_sum_inverse(vec![NestedInteger::Int(1), NestedInteger::List(vec![NestedInteger::Int(4), NestedInteger::List(vec![NestedInteger::Int(6)])])]), 17);
}
