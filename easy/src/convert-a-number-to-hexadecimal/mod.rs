pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".into();
        }

        let mut ans = String::new();
        let mut num = num as i64 & 0xffffffff;

        while num != 0 {
            let r = (num % 16) as u8;
            ans.push(if r >= 10 { (r - 10 + 97) as char } else { (r + 48) as char });
            num /= 16;
        }

        ans.chars().rev().collect::<String>()
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::to_hex(26), "1a".to_string());
    assert_eq!(Solution::to_hex(-1), "ffffffff".to_string());
}
