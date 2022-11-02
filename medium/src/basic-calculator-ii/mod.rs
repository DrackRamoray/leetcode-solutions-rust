struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();
        let mut stack = vec![];
        let mut sign = b'+';
        let mut val = 0;

        for i in 0..n {
            if ss[i] >= b'0' && ss[i] <= b'9' {
                val = val * 10 + ss[i] as i32 - '0' as i32;
            }

            if (ss[i] != b' ' && ss[i] < b'0' || ss[i] > b'9') || i == n - 1 {
                match sign {
                    b'+' => stack.push(val),
                    b'-' => stack.push(-val),
                    b'*' => {
                        if let Some(v) = stack.pop() {
                            stack.push(v * val);
                        }
                    },
                    b'/' => {
                        if let Some(v) = stack.pop() {
                            stack.push(v / val);
                        }
                    },
                    _ => {},
                }

                sign = ss[i];
                val = 0;
            }
        }

        stack.iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::calculate("3+2*2".into()), 7);
        assert_eq!(Solution::calculate(" 3/2 ".into()), 1);
        assert_eq!(Solution::calculate(" 3+5 / 2 ".into()), 5);
    }
}
