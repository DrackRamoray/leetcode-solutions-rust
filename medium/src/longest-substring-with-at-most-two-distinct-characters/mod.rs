pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        if s.len() < 3 {
            return s.len() as i32;
        }

        let s = s.chars().collect::<Vec<char>>();
        let mut l = 0;
        let mut r = 1;
        let mut ans = 2;
        let mut a = 0;
        let mut b = 1;

        while r < s.len() {
            if s[r] == s[a] {
                a = r;
            } else if s[r] == s[b] || s[a] == s[b] {
                b = r;
            } else {
                if a < b {
                    l = a + 1;
                    a = r;
                } else {
                    l = b + 1;
                    b = r;
                };
            }

            r += 1;
            ans = ans.max(r - l);
        }

        ans as i32
    }
}


#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring_two_distinct("eceba".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring_two_distinct("ccaabbb".to_string()), 5);
    }
}
