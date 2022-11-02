pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = std::collections::HashMap::new();

        for str in strs.into_iter() {
            let mut k = str.clone().into_bytes();
            k.sort();
            let entry = ans.entry(k).or_insert(vec![]);
            (*entry).push(str);
        }

        ans.into_values().collect::<Vec<Vec<String>>>()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let strs = vec!["eat".to_owned(), "tea".to_owned(), "tan".to_owned(), "ate".to_owned(), "nat".to_owned(), "bat".to_owned()];
        let mut ans = Solution::group_anagrams(strs);
        ans.sort();
        assert_eq!(ans, vec![vec!["bat".to_owned()], vec!["eat".to_owned(), "tea".to_owned(), "ate".to_owned()], vec!["tan".to_owned(), "nat".to_owned()]]);
        assert_eq!(Solution::group_anagrams(vec!["".to_owned()]), vec![vec!["".to_owned()]]);
        assert_eq!(Solution::group_anagrams(vec!["a".to_owned()]), vec![vec!["a".to_owned()]]);
    }
}
