pub struct Solution;

impl Solution {
    pub fn reverse_words(s: &mut Vec<char>) {
        let len = s.len();
        let mut i = 0;
        let mut j = len - 1;

        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }

        i = 0;
        j = 0;

        while i < len {
            if i == len - 1 || s[i+1] == ' ' {
                s[j..=i].reverse();
                j = i + 2;
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut words = vec!['t','h','e',' ','s','k','y',' ','i','s',' ','b','l','u','e'];
        let ans = vec!['b','l','u','e',' ','i','s',' ','s','k','y',' ','t','h','e'];
        Solution::reverse_words(&mut words);
        assert_eq!(words, ans);

        let mut words = vec!['a'];
        let ans = vec!['a'];
        Solution::reverse_words(&mut words);
        assert_eq!(words, ans);
    }
}
