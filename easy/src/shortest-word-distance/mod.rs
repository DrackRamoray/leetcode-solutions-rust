struct Solution;

impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let n = words_dict.len();
        let mut i = -1 * n as i32;
        let mut j = n as i32;
        let mut ans = n as i32;

        for k in 0..n {
            if words_dict[k] == word1 {
                i = k as i32;
            }
            if words_dict[k] == word2 {
                j = k as i32;
            }

            if i > j {
                ans = ans.min(i - j);
            } else {
                ans = ans.min(j - i);
            }
        }

        ans as i32
    }
}


#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        let words: Vec<String> = vec_stringify!(vec!["practice", "makes", "perfect", "coding", "makes"]);
        let word1 = "coding".to_string();
        let word2 = "practice".to_string();
        assert_eq!(Solution::shortest_distance(words, word1, word2), 3);
        let words: Vec<String> = vec_stringify!(vec!["practice", "makes", "perfect", "coding", "makes"]);
        let word1 = "makes".to_string();
        let word2 = "coding".to_string();
        assert_eq!(Solution::shortest_distance(words, word1, word2), 1);
    }
}
