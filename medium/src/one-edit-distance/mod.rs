pub struct Solution;

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        if s.len() < t.len() {
            return Self::is_one_edit_distance(t, s);
        }

        if s.len() - t.len() > 1 {
            return false;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..t.len() {
            if s[i] != t[i] {
                if s.len() == t.len() {
                    return s[i+1..] == t[i+1..];
                }

                return s[i+1..] == t[i..];
            }
        }

        s.len() == t.len() + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_one_edit_distance("ab".to_string(), "acb".to_string()), true);
        assert_eq!(Solution::is_one_edit_distance("cab".to_string(), "ad".to_string()), false);
    }
}
