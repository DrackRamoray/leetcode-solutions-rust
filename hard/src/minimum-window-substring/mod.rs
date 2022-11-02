struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut mp = std::collections::HashMap::new();

        for u in t.as_bytes() {
            *mp.entry(u).or_insert(0) += 1;
        }

        let ss = s.as_bytes();
        let mut i = 0;
        let mut n = ss.len();
        let mut cnt = t.len();
        let mut start = 0;
        let mut end = 0;

        for j in 0..n {
            if let Some(p) = mp.get_mut(&ss[j]) {
                *p -= 1;

                if *p >= 0 {
                    cnt -= 1;
                }
            }

            while cnt == 0 {
                if j - i < n {
                    n = j - i;
                    start = i;
                    end = j + 1;
                }

                if let Some(p) = mp.get_mut(&ss[i]) {
                    *p += 1;

                    if *p > 0 {
                        cnt += 1;
                    }
                }

                i += 1;
            }
        }

        s[start..end].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a".to_string());
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "".to_string());
    }
}
