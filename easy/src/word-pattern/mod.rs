use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut p2s = HashMap::new();
        let mut s2p = HashMap::new();
        let mut pv = pattern.chars();
        let mut sv = s.split_whitespace();

        loop {
            match (pv.next(), sv.next()) {
                (Some(c), Some(w)) => {
                    if let Some(ww) = p2s.insert(c, w) {
                        if ww != w {
                            return false;
                        }
                    }

                    if let Some(cc) = s2p.insert(w, c) {
                        if cc != c {
                            return false;
                        }
                    }
                },
                (None, None) => break,
                _ => return false,
            }
        }

        true
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::word_pattern("abba".into(), "dog cat cat dog".into()), true);
    assert_eq!(Solution::word_pattern("abba".into(), "dog cat cat fish".into()), false);
    assert_eq!(Solution::word_pattern("aaaa".into(), "dog cat cat dog".into()), false);
}
