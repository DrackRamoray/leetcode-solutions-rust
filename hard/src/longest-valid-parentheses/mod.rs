pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];
        stack.push(-1);
        let n = s.len();
        let ss = s.as_bytes();

        for i in 0..n {
            if ss[i] == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    ans = ans.max(i as i32 - stack[stack.len()-1]);
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_owned()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_owned()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_owned()), 0);
    }
}
