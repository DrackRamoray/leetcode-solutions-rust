struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let n = strs.len();

        if n < 1 {
            return "".to_owned();
        }

        let mut prefix = strs[0].as_str();

        for i in 1..n {
            prefix = Solution::lcp(prefix, &strs[i]);
        }

        prefix.into()
    }

    fn lcp<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        let len = str1.len().min(str2.len());
        let mut ans = 0_usize;

        while ans < len && &str1[ans..ans+1] == &str2[ans..ans+1] {
            ans += 1;
        }

        return &str1[0..ans]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::longest_common_prefix(vec!["flower".to_owned(),"flow".to_owned(),"flight".to_owned()]), "fl".to_owned());
        assert_eq!(Solution::longest_common_prefix(vec!["dog".to_owned(),"racecar".to_owned(),"car".to_owned()]), "".to_owned());
    }
}
