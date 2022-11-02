use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = HashSet::new();

        for w in word_list {
            words.insert(w);
        }

        if !words.contains(&end_word) {
            return 0;
        }

        let mut step = 1;
        let mut q = vec![begin_word.to_string()];
        let mut vis = HashSet::new();
        vis.insert(begin_word);
        let characters = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

        while q.len() > 0 {
            let q_size = q.len();

            for _ in 0..q_size {
                let word = q.remove(0);

                for j in 0..word.len() {
                    for k in &characters {
                        let new_word = (&word[0..j]).to_string() + &(k.to_string()) + &word[j+1..];

                        if new_word == end_word {
                            return step + 1;
                        }

                        if !words.contains(&new_word) || new_word == word {
                            continue;
                        }

                        if !vis.contains(&new_word) {
                            q.push(new_word.to_string());
                            vis.insert(new_word);
                        }
                    }
                }
            }

            step += 1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let words = ["hot","dot","dog","lot","log","cog"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::ladder_length("hit".into(), "cog".into(), words), 5);

        let words = ["hot","dot","dog","lot","log"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::ladder_length("hit".into(), "cog".into(), words), 0);
    }
}
