#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

pub struct Solution;

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut ss = s.chars();
        let mut neg = 1;
        let mut num: Option<i32> = None;
        let mut stack = vec![];

        while let Some(c) = ss.next() {
            match c {
                '-' => {
                    neg = -1;
                },
                '[' => {
                    stack.push(NestedInteger::List(vec![]));
                },
                ',' | ']' => {
                    if let Some(v) = num {
                        if let Some(NestedInteger::List(ref mut items)) = stack.last_mut() {
                            items.push(NestedInteger::Int(neg * v));
                        }
                    }

                    num = None;
                    neg = 1;

                    if c == ']' && stack.len() > 1 {
                        if let Some(item) = stack.pop() {
                            if let Some(NestedInteger::List(ref mut items)) = stack.last_mut() {
                                items.push(item);
                            }
                        }
                    }
                },
                '0'..='9' => {
                    if let Some(v) = num {
                        num = Some(v * 10 + (c as i32 - '0' as i32));
                    } else {
                        num = Some(c as i32 - '0' as i32);
                    }
                },
                _ => {}
            }
        }


        if let Some(v) = stack.pop() {
            v
        } else if let Some(v) = num{
            NestedInteger::Int(neg * v)
        } else {
            NestedInteger::Int(-1000001)
        }
    }
}

#[test]
fn it_works() {
    let s = "324".to_string();
    let res = Solution::deserialize("324".to_string());
    assert_eq!(Solution::deserialize(s), res);
    let s = "[123,[456,[789]]]".to_string();
    let res = Solution::deserialize("[123,[456,[789]]]".to_string());
    assert_eq!(Solution::deserialize(s), res);
    let s = "[123,456,[788,799,833],[[]],10,[]]".to_string();
    let res = Solution::deserialize("[123,456,[788,799,833],[[]],10,[]]".to_string());
    assert_eq!(Solution::deserialize(s), res);
}
