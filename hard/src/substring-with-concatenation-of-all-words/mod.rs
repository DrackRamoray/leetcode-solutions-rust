use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut ans = Vec::new();
        let n = s.len();
        let m = words.len();

        if m == 0 {
            return ans;
        }

        let v = words[0].len();

        let mut g = HashMap::new();

        for i in 0..m {
            let k = g.entry(words[i].clone()).or_insert(0);
            *k += 1;
        }

        for i in 0..(n - m * v + 1) {
            let mut t = HashMap::new();
            let mut j = 0;

            while j < m {
                let w = s[(i + j * v)..(i + (j + 1) * v)].to_string();

                if !g.contains_key(&w) {
                    break;
                }

                let k = t.entry(w.clone()).or_insert(0);
                *k += 1;

                if *k > *(g.get(&w).unwrap()) {
                    break;
                }

                j += 1;
            }

            if j == m {
                ans.push(i as i32);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_substring("barfoothefoobarman".to_owned(), vec!["foo".to_owned(), "bar".to_owned()]), vec![0,9]);
        assert_eq!(Solution::find_substring("wordgoodgoodgoodbestword".to_owned(), vec!["word".to_owned(), "good".to_owned(), "best".to_owned(), "word".to_owned()]), vec![]);
        assert_eq!(Solution::find_substring("barfoofoobarthefoobarman".to_owned(), vec!["bar".to_owned(), "foo".to_owned(), "the".to_owned()]), vec![6,9,12]);
    }
}
