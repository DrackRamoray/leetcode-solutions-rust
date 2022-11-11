pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let n = s.len();
        let m = t.len();

        if n > m {
            return false;
        }

        let mut i = 0_usize;
        let mut j = 0_usize;

        let s1 = s.as_bytes();
        let t1 = t.as_bytes();

        while i < n && j < m {
            if s1[i] == t1[j] {
                i += 1;
            }

            j += 1;
        }

        return i == n;
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
    assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
}
