pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let len = num.len();
        let mut k = k as usize;
        let mut pos = 1;
        let mut stack = vec![];
        let s = num.as_bytes();
        stack.push(s[0]);

        while pos < len {
            while k > 0 && stack.len() > 0 && stack[stack.len() - 1] > s[pos] {
                k -= 1;
                stack.pop();
            }

            stack.push(s[pos]);
            pos += 1;
        }

        let res = stack[0..(stack.len()-k)].iter().skip_while(|&&c| c == b'0').map(|&c| c as char).collect::<String>();

        if res.len() == 0 {
            return "0".into();
        }

        res
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::remove_kdigits("1432219".to_string(), 3), "1219".to_string());
    assert_eq!(Solution::remove_kdigits("10200".to_string(), 1), "200".to_string());
    assert_eq!(Solution::remove_kdigits("10".to_string(), 2), "0".to_string());
}
