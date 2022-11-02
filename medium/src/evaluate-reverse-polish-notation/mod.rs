struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for token in tokens.iter() {
            match token.as_ref() {
                "+" => match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        stack.push(a+b);
                    },
                    _ => {},
                },
                "-" => match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        stack.push(b-a);
                    },
                    _ => {},
                }
                "*" => match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        stack.push(a*b);
                    },
                    _ => {},
                },
                "/" => match (stack.pop(), stack.pop()) {
                    (Some(a), Some(b)) => {
                        stack.push(b/a);
                    },
                    _ => {},
                },
                num @ _ => {
                    stack.push(num.parse::<i32>().unwrap());
                },
            }
        }

        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        let tokens = vec_stringify!(["2","1","+","3","*"]);
        assert_eq!(Solution::eval_rpn(tokens), 9);

        let tokens = vec_stringify!(["4","13","5","/","+"]);
        assert_eq!(Solution::eval_rpn(tokens), 6);

        let tokens = vec_stringify!(["10","6","9","3","+","-11","*","/","*","17","+","5","+"]);
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
