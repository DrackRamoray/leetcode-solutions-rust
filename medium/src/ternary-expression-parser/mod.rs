pub struct Solution;

impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let n = expression.len();
        let mut expr = expression.chars().collect::<Vec<char>>();
        let mut stack = vec![];
        let mut i = n - 1;

        while i >= 2 {
            if expr[i-1] == ':' {
                stack.push(expr[i]);
            } else {
                if expr[i-2] == 'T' {
                    expr[i-2] = expr[i];
                } else {
                    expr[i-2] = stack[stack.len()-1];
                }

                stack.pop();
            }

            i -= 2;
        }

        expr[0].to_string()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::parse_ternary("T?2:3".to_string()), "2".to_string());
    assert_eq!(Solution::parse_ternary("F?1:T?4:5".to_string()), "4".to_string());
    assert_eq!(Solution::parse_ternary("T?T?F:5:3".to_string()), "F".to_string());
}
