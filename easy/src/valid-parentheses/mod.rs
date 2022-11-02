pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for u in s.bytes() {
            match u {
                c @ (b'(' | b'[' | b'{') => {
                    stack.push(c);
                },
                b')' if stack.len() > 0 && stack[stack.len() - 1] == b'(' => {
                    stack.pop();
                },
                b']' if stack.len() > 0 && stack[stack.len() - 1] == b'[' => {
                    stack.pop();
                },
                b'}' if stack.len() > 0 && stack[stack.len() - 1] == b'{' => {
                    stack.pop();
                },
                _ => {
                    return false;
                },
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }
}
