pub struct Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let words = word.chars().collect::<Vec<char>>();
        let len = words.len();
        let mut digit = 0;
        let mut total = 0;

        for ch in abbr.chars() {
            if ch >= 'a' && ch <= 'z' {
                total += digit + 1;

                if total > len || words[total - 1] != ch {
                    return false;
                }

                digit = 0;
            } else if digit == 0 && ch =='0' {
                return false
            } else {
                digit = digit * 10 + ((ch as usize) - 48);
            }
        }

        total + digit == len
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::valid_word_abbreviation("internationalization".to_string(), "i12iz4n".to_string()), true);
    assert_eq!(Solution::valid_word_abbreviation("apple".to_string(), "a2e".to_string()), false);
}
