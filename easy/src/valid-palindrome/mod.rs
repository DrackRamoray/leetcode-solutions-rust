pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes = s.into_bytes();
        let mut lo = 0;
        let mut hi = bytes.len() - 1;

        while lo < hi {
            let t1 = bytes[lo].is_ascii_alphanumeric();
            let t2 = bytes[hi].is_ascii_alphanumeric();

            if t1 && t2 {
                if bytes[lo].to_ascii_lowercase() != bytes[hi].to_ascii_lowercase() {
                    return false;
                }

                lo += 1;
                hi -= 1;
            }

            if !t1 {
                lo += 1;
            }

            if !t2 {
                hi -= 1;
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
        assert_eq!(Solution::is_palindrome("A man, a plan, a canal: Panama".into()), true);
        assert_eq!(Solution::is_palindrome("race a car".into()), false);
        assert_eq!(Solution::is_palindrome(" ".into()), true);
    }
}
