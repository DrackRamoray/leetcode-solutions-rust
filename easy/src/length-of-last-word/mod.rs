pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let n = s.len();
        let ss = s.as_bytes();
        let mut cnt = 0;

        for i in (0..n).rev() {
            if ss[i] != b' ' {
                cnt += 1;
            } else if cnt != 0 {
                break;
            }
        }

        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_owned()), 5);
        assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_owned()), 4);
        assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_owned()), 6);
    }
}
