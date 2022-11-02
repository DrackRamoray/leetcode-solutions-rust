pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let ss = s.as_bytes();
        let mut i = 0;
        let mut mp = std::collections::HashMap::new();
        let mut ans = 0;

        for j in 0..n {
            if let Some(k) = mp.get(&ss[j]) {
                i = i.max(k + 1);
            }

            ans = ans.max(j - i + 1);
            mp.insert(ss[j], j);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_owned()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbbb".to_owned()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_owned()), 3);
    }
}
