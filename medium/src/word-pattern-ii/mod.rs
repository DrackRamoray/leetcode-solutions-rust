use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        let mut p2s = HashMap::new();
        let mut s2p = HashMap::new();

        Self::track_back(&pattern, &s, &mut p2s, &mut s2p)
    }

    fn track_back(p: &str, s: &str, p2s: &mut HashMap<String, String>, s2p: &mut HashMap<String, String>) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }

        if s.len() == 0 {
            return p.len() == 0;
        }

        let c = p.get(0..=0).unwrap();

        for i in 1..=(s.len() - p.len() + 1) {
            let ss = s.get(0..i).unwrap();

            if let Some(ms) = p2s.get(c) {
                if ms == ss && Self::track_back(&p[1..], &s[i..], p2s, s2p)  {
                    return true;
                }
            } else if !s2p.contains_key(ss) {
                p2s.insert(c.to_string(), ss.to_string());
                s2p.insert(ss.to_string(), c.to_string());

                if Self::track_back(&p[1..], &s[i..], p2s, s2p) {
                    return true;
                }

                p2s.remove(c);
                s2p.remove(ss);
            }
        }

        false
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::word_pattern_match("abab".to_string(), "redblueredblue".to_string()), true);
    assert_eq!(Solution::word_pattern_match("aaaa".to_string(), "asdasdasdasd".to_string()), true);
    assert_eq!(Solution::word_pattern_match("aabb".to_string(), "xyzabcxzyabc".to_string()), false);
}
