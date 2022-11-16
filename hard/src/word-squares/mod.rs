use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn word_squares(words: Vec<String>) -> Vec<Vec<String>> {
        let n = words.len();
        let m = words[0].len();
        let words: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let mut prefix: HashMap<Vec<char>, Vec<&[char]>> = HashMap::new();

        for word in words.iter() {
            for i in 0..=m {
                prefix.entry(word[..i].to_vec()).or_default().push(word);
            }
        }

        let mut ans = vec![];

        Self::dfs(&prefix, &mut ans, &mut Vec::new(), n, m, 0);

        ans
    }

    fn dfs<'a>(prefix: &HashMap<Vec<char>, Vec<&'a [char]>>, ans: &mut Vec<Vec<String>>, selected: &mut Vec<&'a [char]>, n: usize, m: usize, begin: usize) {
        if begin == m {
            ans.push((0..m).map(|i| selected[i].iter().collect()).collect());
        } else {
            let mut tmp = vec![];

            for i in 0..begin {
                tmp.push(selected[i][begin]);
            }

            if let Some(candidates) = prefix.get(&tmp) {
                for word in candidates {
                    selected.push(word);
                    Self::dfs(prefix, ans, selected, n, m, begin + 1);
                    selected.pop();
                }
            }
        }
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::word_squares(vec_stringify!(["area","lead","wall","lady","ball"])), vec![vec_stringify!(["wall","area","lead","lady"]),vec_stringify!(["ball","area","lead","lady"])]);
    assert_eq!(Solution::word_squares(vec_stringify!(["abat","baba","atan","atal"])), vec![vec_stringify!(["baba","abat","baba","atan"]),vec_stringify!(["baba","abat","baba","atal"])]);
}
