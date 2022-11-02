use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut ss = HashSet::new();

        for c in s.chars() {
            if ss.contains(&c) {
                ss.remove(&c);
            } else {
                ss.insert(c);
            }
        }

        ss.len() <= 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::can_permute_palindrome("code".into()), false);
    assert_eq!(Solution::can_permute_palindrome("aab".into()), true);
    assert_eq!(Solution::can_permute_palindrome("carerac".into()), true);
}
