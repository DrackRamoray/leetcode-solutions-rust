pub struct Solution;

impl Solution {
    pub fn find_ladders(begin_word: String, end_word: String, mut word_list: Vec<String>) -> Vec<Vec<String>> {
        let k = match word_list.iter().position(|w| *w == end_word) {
            Some(i) => i,
            None => return Vec::new(),
        };

        word_list.push(begin_word);

        let mut prev = vec![(Vec::new(), -1); word_list.len()];
        prev[word_list.len() - 1].1 = 0;

        for depth in 0.. {
            let mut advanced = false;
            for (i, w) in word_list.iter().enumerate() {
                if prev[i].1 == depth {
                    for (j, w1) in word_list.iter().enumerate() {
                        if (prev[j].1 == -1 || prev[j].1 == depth + 1) && Self::check(w, w1) {
                            advanced = true;
                            prev[j].0.push(i);
                            prev[j].1 = depth + 1;
                        }
                    }
                }
            }

            if !advanced {
                return Vec::new();
            }

            if prev[k].1 != -1 {
                break;
            }
        }

        Self::output(k, &word_list, &prev)
    }

    fn check(a: &str, b: &str) -> bool {
        a.chars().zip(b.chars()).filter(|(c1, c2)| c1 != c2).count() == 1
    }

    fn output(k: usize, ws: &[String], prev: &[(Vec<usize>, i32)]) -> Vec<Vec<String>> {
        let w = &ws[k];

        if k == ws.len() - 1 {
            return vec![vec![w.to_string()]];
        }

        prev[k].0.iter().flat_map(|&v| Self::output(v, ws, prev)).map(|mut v| {
            v.push(w.to_string());
            v
        })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let words = ["hot","dot","dog","lot","log","cog"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        let ans = vec![
            ["hit","hot","dot","dog","cog"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
            ["hit","hot","lot","log","cog"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        ];
        assert_eq!(Solution::find_ladders("hit".into(), "cog".into(), words), ans);

        let words = ["hot","dot","dog","lot","log"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        let ans = vec![] as Vec<Vec<String>>;
        assert_eq!(Solution::find_ladders("hit".into(), "cog".into(), words), ans);
    }
}
