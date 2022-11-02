pub struct WordDistance {
    words: Vec<String>,
}

impl WordDistance {

    fn new(words_dict: Vec<String>) -> Self {
        Self {
            words: words_dict,
        }
    }

    fn shortest(&self, word1: String, word2: String) -> i32 {
        let mut i = -1;
        let mut j = -1;
        let mut ans = self.words.len() as i32;

        for (n, w) in self.words.iter().enumerate() {
            if w == &word1 {
                i = n as i32;
            } else if w == &word2 {
                j = n as i32;
            }

            if i != - 1 && j != -1 {
                ans = ans.min((i - j).abs());
            }
        }

        ans
    }
}


#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::WordDistance;

    #[test]
    fn it_works() {
        let words = vec_stringify!(vec!["practice", "makes", "perfect", "coding", "makes"]);
        let obj = WordDistance::new(words);
        let word1 = "coding".to_string();
        let word2 = "practice".to_string();
        let res = 3;
        assert_eq!(obj.shortest(word1, word2), res);
        let word1 = "makes".to_string();
        let word2 = "coding".to_string();
        let res = 1;
        assert_eq!(obj.shortest(word1, word2), res);
    }
}
