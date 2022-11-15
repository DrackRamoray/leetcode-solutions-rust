pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = vec![0;52];

        for ch in s.chars() {
            if ch.is_ascii_uppercase() {
                cnt[ch as usize - 65] += 1;
            } else {
                cnt[ch as usize - 71] += 1;
            }
        }

        let mut ans = 0;

        for num in cnt.iter() {
            ans += num / 2 * 2;
            if num % 2 == 1 && ans % 2 == 0 {
                ans += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    assert_eq!(Solution::longest_palindrome("aaaaaccc".to_string()), 7);
}
