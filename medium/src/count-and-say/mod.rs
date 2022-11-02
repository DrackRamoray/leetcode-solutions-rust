pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = vec!["1".to_string();1];

        for _ in 1..n {
            let mut k = 0;
            let mut j = 0;
            let mut tmp = Vec::new();

            while j < s.len() {
                if s[k] == s[j] {
                    j += 1;
                } else {
                    tmp.push((j - k).to_string());
                    tmp.push(s[k].to_string());
                    k = j;
                }
            }

            tmp.push((j - k).to_string());
            tmp.push(s[k].to_string());
            s = tmp;
        }

        return s.join("");
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_and_say(1), "1".to_owned());
        assert_eq!(Solution::count_and_say(4), "1211".to_owned());
    }
}
