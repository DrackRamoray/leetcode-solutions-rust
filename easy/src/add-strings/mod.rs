pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let n = num1.len();
        let m = num2.len();
        let mut ans = vec![];
        let mut i = n - 1;
        let mut j = m - 1;
        let mut c = 0;
        let n1 = num1.as_bytes();
        let n2 = num2.as_bytes();

        while i < n && j < m {
            let tmp = n1[i] as i32 + n2[j] as i32 + c - 96;
            ans.push(tmp % 10);
            c = tmp / 10;
            i = i.wrapping_sub(1);// i -= 1;
            j = j.wrapping_sub(1);// j -= 1;
        }

        while i < n {
            let tmp = n1[i] as i32 + c - 48;
            ans.push(tmp % 10);
            c = tmp / 10;
            i = i.wrapping_sub(1);// i -= 1;
        }

        while j < m {
            let tmp = n2[j] as i32 + c - 48;
            ans.push(tmp % 10);
            c = tmp / 10;
            j = j.wrapping_sub(1);// j -= 1;
        }

        if c != 0 {
            ans.push(c);
        }

        ans.into_iter().rev().map(|x| x.to_string()).collect::<String>()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134".to_string());
    assert_eq!(Solution::add_strings("456".to_string(), "77".to_string()), "533".to_string());
    assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0".to_string());
}
