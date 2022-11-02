struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        let mut next = vec![-1;n];

        for i in 1..n {
            let mut j = next[i-1];
            while j != -1 && s[(j+1) as usize..=(j+1) as usize] != s[i..=i] {
                j = next[j as usize];
            }
            if s[(j+1) as usize..=(j+1) as usize] == s[i..=i] {
                next[i] = j + 1;
            }
        }

        let mut j = -1;

        for i in (0..n).rev() {
            while j != - 1 && s[(j+1) as usize..=(j+1) as usize] != s[i..=i] {
                j = next[j as usize];
            }
            if s[(j+1) as usize..=(j+1) as usize] == s[i..=i] {
                j += 1;
            }
        }

        let tmp = if j == n as i32 - 1 {
            "".to_string()
        } else {
            (&s[j as usize+1..]).chars().rev().collect::<String>()
        };

        tmp + &s
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::shortest_palindrome("aacecaaa".to_string()), "aaacecaaa".to_string());
        assert_eq!(Solution::shortest_palindrome("abcd".to_string()), "dcbabcd".to_string());
    }
}
