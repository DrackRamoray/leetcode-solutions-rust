pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Start;

        for &u in s.as_bytes() {
            match state {
                State::Start => {
                    match u {
                        b' ' => {},
                        b'+' => {
                            state = State::Positive(0);
                        },
                        b'-' => {
                            state = State::Negative(0);
                        },
                        b'0'..=b'9' => {
                            state = State::Positive(u as i32 - '0' as i32);
                        },
                        _ => {
                            state = State::End(0);
                        },
                    };
                },
                State::Positive(num) => {
                    if b'0' <= u && u <= b'9' {
                        let v = u as i32 - '0' as i32;

                        if num > i32::MAX / 10 || (num == i32::MAX / 10 && v > 7)  {
                            state = State::End(i32::MAX);
                        } else {
                            state = State::Positive(num * 10 + v);
                        }
                    } else {
                        state = State::End(num);
                    }
                },
                State::Negative(num) => {
                    if b'0' <= u && u <= b'9' {
                        let v = u as i32 - '0' as i32;

                        if num < i32::MIN / 10 || (num == i32::MIN / 10 && v > 7)  {
                            state = State::End(i32::MIN);
                        } else {
                            state = State::Negative(num * 10 - v);
                        }
                    } else {
                        state = State::End(num);
                    }
                },
                State::End(_) => {
                    break;
                },
            }
        }

        match state {
            State::Start => 0,
            State::Positive(num) | State::Negative(num) | State::End(num) => num,
        }
    }
}

enum State {
    Start,
    Positive(i32),
    Negative(i32),
    End(i32),
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::my_atoi("42".to_owned()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_owned()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_owned()), 4193);
    }
}
