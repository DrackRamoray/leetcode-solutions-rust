pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut ss = s.chars().collect::<Vec<_>>();
        let n = ss.len();
        let mut i = 0;
        let mut j = n - 1;

        while i < j && j < n {
            if Self::is_vowels(ss[i]) && Self::is_vowels(ss[j]) {
                ss.swap(i, j);
                i += 1;
                j -= 1;
                continue;
            }
            if !Self::is_vowels(ss[i]) {
                i += 1;
            }
            if !Self::is_vowels(ss[j]) {
                j -= 1;
            }
        }

        ss.into_iter().collect::<String>()
    }

    fn is_vowels(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
}
