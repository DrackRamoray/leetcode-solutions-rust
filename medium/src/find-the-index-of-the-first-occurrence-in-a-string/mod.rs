use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let m = haystack.len();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        if m < n {
            return -1;
        }

        let mut shift = HashMap::new();

        for i in (0..n).rev() {
            if !shift.contains_key(&needle[i]) {
                shift.insert(needle[i], n - i);
            }
        }

        let mut idx = 0;

        while idx <= m - n {
            let hay = &haystack[idx..idx + n];

            if hay == needle {
                return idx as i32;
            }

            if idx >= m - n {
                return -1;
            }

            idx = match shift.get(&haystack[idx + n]) {
                Some(v) => idx + v,
                None => idx + n + 1
            }
        }

        if idx + n > m {
            -1
        } else {
            idx as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::str_str("sadbutsad".to_owned(), "sad".to_owned()), 0);
        assert_eq!(Solution::str_str("leetcode".to_owned(), "leeto".to_owned()), -1);
    }
}
