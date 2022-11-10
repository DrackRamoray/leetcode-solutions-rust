pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ans = 0 as i32;

        for byt in t.bytes() {
            ans += byt as i32;
        }

        for byt in s.bytes() {
            ans -= byt as i32;
        }

        ans as u8 as char
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_the_difference("abcd".to_string(), "abcde".to_string()), 'e');
    assert_eq!(Solution::find_the_difference("".to_string(), "y".to_string()), 'y');
}
