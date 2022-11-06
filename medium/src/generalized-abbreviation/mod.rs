pub struct Solution;

impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let words = word.chars().collect::<Vec<char>>();
        let mut ans = vec![];

        Self::dfs(&words, &mut ans, &mut String::new(), 0, 0);

        ans
    }

    fn dfs(words: &Vec<char>, ans: &mut Vec<String>, selected: &mut String, cur: usize, cnt: u8) {
        let n = selected.len();

        if cur == words.len() {
            if cnt > 0 {
                selected.push_str(&cnt.to_string());
            }

            ans.push(selected.clone());
        } else {
            Self::dfs(words, ans, selected, cur + 1, cnt + 1);

            if cnt > 0 {
                selected.push_str(&cnt.to_string());
            }

            selected.push(words[cur]);

            Self::dfs(words, ans, selected, cur + 1, 0);
        }

        selected.truncate(n);
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::generate_abbreviations("word".to_string()), vec_stringify!(["4","3d","2r1","2rd","1o2","1o1d","1or1","1ord","w3","w2d","w1r1","w1rd","wo2","wo1d","wor1","word"]));
    assert_eq!(Solution::generate_abbreviations("a".to_string()), vec_stringify!(["1", "a"]))
}
