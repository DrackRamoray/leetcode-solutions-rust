struct Solution;

impl Solution {
    pub fn shortest_word_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let mut n = -1;
        let mut m = -1;
        let mut ans = words_dict.len() as i32;

        for (i, w) in words_dict.into_iter().enumerate() {
            if w == word1 {
                n = if word1 == word2 {
                    m
                } else {
                    i as i32
                };
            }

            if w == word2 {
                m = i as i32;
            }

            if n != -1 && m != -1 && n != m {
                ans = ans.min((n - m).abs());
            }
        }

        ans
    }
}


#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::shortest_word_distance(vec_stringify!(vec!["practice", "makes", "perfect", "coding", "makes"]), "makes".into(), "coding".into()), 1);
        assert_eq!(Solution::shortest_word_distance(vec_stringify!(vec!["practice", "makes", "perfect", "coding", "makes"]), "makes".into(), "makes".into()), 3);
    }
}
