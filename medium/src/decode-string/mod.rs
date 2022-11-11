pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = vec![];
        let mut is_numeric = false;

        for ch in s.chars() {
            if ch.is_ascii_lowercase() || ch == '[' {
                is_numeric = false;
                stack.push(ch.to_string());
            } else if ch.is_ascii_digit() {
                if is_numeric {
                    let num = format!("{}{}", stack[stack.len() - 1], ch);
                    let index = stack.len() - 1;
                    stack[index] = num;
                } else {
                    stack.push(ch.to_string());
                }

                is_numeric = true;
            } else {
                let mut tmp = vec![];

                while stack.len() > 0 && stack[stack.len() - 1] != "[" {
                    if let Some(c) = stack.pop() {
                        tmp.push(c);
                    }
                }

                tmp.reverse();
                stack.pop();

                if let Some(num) = stack.pop() {
                    if let Ok(num) = num.parse::<usize>() {
                        stack.push(tmp.join("").repeat(num));
                    }
                }

                is_numeric = false;
            }
        }

        stack.join("")
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::decode_string("3[a]2[bc]".to_string()), "aaabcbc".to_string());
    assert_eq!(Solution::decode_string("3[a2[c]]".to_string()), "accaccacc".to_string());
    assert_eq!(Solution::decode_string("2[abc]3[cd]ef".to_string()), "abcabccdcdcdef".to_string());
    assert_eq!(Solution::decode_string("abc3[cd]xyz".to_string()), "abccdcdcdxyz".to_string());
}
