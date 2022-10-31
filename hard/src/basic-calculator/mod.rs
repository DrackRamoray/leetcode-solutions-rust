pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut ops = vec![1];
        let mut sign = 1;
        let mut ans = 0;
        let n = s.len();
        let mut c = 0;
        let mut i =0;

        while i < n {
            match ss[i] {
                b' ' => {
                    i += 1;
                },
                b'+' => {
                    sign = ops[c];
                    i += 1;
                },
                b'-' => {
                    sign = -ops[c];
                    i += 1;
                }
                b'(' => {
                    ops.push(sign);
                    c += 1;
                    i += 1;
                },
                b')' => {
                    ops.pop();
                    c -= 1;
                    i += 1;
                },
                _ => {
                    let mut tmp = 0;

                    while i < n && ss[i] >= b'0' && ss[i] <= b'9' {
                        tmp = tmp * 10 + ss[i] as i32 - '0' as i32;
                        i += 1;
                    }

                    ans += sign * tmp;
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
        let s = "1 + 1".to_string();
        let ans = 2;
        assert_eq!(Solution::calculate(s), ans);
        let s = " 2-1 + 2 ".to_string();
        let ans = 3;
        assert_eq!(Solution::calculate(s), ans);
        let s = "(1+(4+5+2)-3)+(6+8)".to_string();
        let ans = 23;
        assert_eq!(Solution::calculate(s), ans);
    }
}
