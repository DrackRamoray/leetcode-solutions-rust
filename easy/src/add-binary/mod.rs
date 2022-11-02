struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let n = a.len();
        let m = b.len();
        let c = if n >= m { n + 1 } else { m + 1 };
        let mut ans = vec![0;c];
        let mut aa = a.bytes().rev();
        let mut bb = b.bytes().rev();
        let mut carry = 0;

        for i in (0..c).rev() {
            match (aa.next(), bb.next()) {
                (Some(ua), Some(ub)) => {
                    let tmp = (ua as i32) + (ub as i32) - 96 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                (Some(ua), None) => {
                    let tmp = (ua as i32) - 48 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                (None, Some(ub)) => {
                    let tmp = (ub as i32) - 48 + carry;
                    ans[i] = tmp % 2;
                    carry = tmp / 2;
                },
                _ => {
                    if carry != 0 {
                        ans[i] = carry;
                    }
                },
            }
        }

        let mut skip = 0;

        for i in 0..c-1 {
            if ans[i] == 0 {
                skip += 1;
            } else {
                break;
            }
        }

        ans[skip..].into_iter().map(|x| x.to_string()).collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::add_binary("11".to_string(), "1".to_string()), "100".to_string());
        assert_eq!(super::Solution::add_binary("1010".to_string(), "1011".to_string()), "10101".to_string());
    }
}
