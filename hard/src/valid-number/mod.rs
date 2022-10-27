pub struct Solution;

enum State {
    Begin,
    BaseInt(bool),
    BaseFloat(bool),
    Exp(bool),
    ExpVal(bool),
    Trailing(bool),
}

impl Solution {
    pub fn is_number(s: String) -> bool {
        use State::*;

        let mut state = Begin;

        for c in s.chars() {
            state = match state {
                Begin => match c {
                    ' ' => Begin,
                    '-'|'+' => BaseInt(false),
                    '.' => BaseFloat(false),
                    '0'..='9' => BaseInt(true),
                    _ => return false,
                },
                BaseInt(v) => match c {
                    '0'..='9' => BaseInt(true),
                    '.' => BaseFloat(v),
                    'e'|'E' => Exp(v),
                    ' ' => Trailing(v),
                    _ => return false,
                },
                BaseFloat(v) => match c {
                    '0'..='9' => BaseFloat(true),
                    'e'|'E' => Exp(v),
                    ' ' => Trailing(v),
                    _ => return false,
                },
                Exp(v) => match c {
                    '0'..='9' if v => ExpVal(true),
                    '-'|'+' if v  => ExpVal(false),
                    _ => return false,
                },
                ExpVal(v) => match c {
                    '0'..='9' => ExpVal(true),
                    ' ' => Trailing(v),
                    _ => return false,
                },
                Trailing(v) => match c {
                    ' ' => Trailing(v),
                    _ => return false,
                },
            };
        }

        match state {
            Trailing(v) | BaseInt(v) | BaseFloat(v) | ExpVal(v) => v,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number("e".to_string()), false);
        assert_eq!(Solution::is_number(".".to_string()), false);
    }
}
