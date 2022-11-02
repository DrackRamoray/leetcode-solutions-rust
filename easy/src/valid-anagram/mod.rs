struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut vec = s.bytes().fold(vec![0;26], |mut acc, u| {
            acc[u as usize - b'a' as usize] += 1;
            acc
        });

        for u in t.bytes() {
            vec[u as usize - b'a' as usize] -= 1;

            if vec[u as usize - b'a' as usize] < 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_anagram("anagram".into(), "nagaram".into()), true);
        assert_eq!(Solution::is_anagram("rat".into(), "car".into()), false);
    }
}
