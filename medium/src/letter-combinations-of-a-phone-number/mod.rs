struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = vec![];

        if digits.len() == 0 {
            return ans;
        }

        Self::dfs(digits.as_bytes(), &mut ans, &mut String::new(), 0);

        ans
    }

    fn dfs(ss: &[u8], ans: &mut Vec<String>, selected: &mut String, begin: usize) {
        if begin == ss.len() {
            ans.push(selected.to_string());
            return;
        }

        let letters = Self::get_letters(ss[begin]);

        for &letter in letters.iter() {
            selected.push(letter);
            Self::dfs(ss, ans, selected, begin + 1);
            selected.pop();
        }
    }

    fn get_letters(num: u8) -> Vec<char> {
        match num {
            b'2' => vec!['a', 'b', 'c'],
            b'3' => vec!['d', 'e', 'f'],
            b'4' => vec!['g', 'h', 'i'],
            b'5' => vec!['j', 'k', 'l'],
            b'6' => vec!['m', 'n', 'o'],
            b'7' => vec!['p', 'q', 'r', 's'],
            b'8' => vec!['t', 'u', 'v'],
            b'9' => vec!['w', 'x', 'y', 'z'],
            _ => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::letter_combinations("23".to_owned()), ["ad","ae","af","bd","be","bf","cd","ce","cf"].into_iter().map(|v| v.to_owned()).collect::<Vec<_>>());
        assert_eq!(Solution::letter_combinations("".to_owned()), vec![] as Vec<String>);
        assert_eq!(Solution::letter_combinations("2".to_owned()), vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]);
    }
}
