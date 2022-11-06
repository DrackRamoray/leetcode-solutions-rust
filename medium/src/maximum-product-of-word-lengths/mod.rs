pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let n = words.len();
        let masks = words.iter().map(|word| word.bytes().fold(0, |m, w| m | 1 << (w as usize - 'a' as usize))).collect::<Vec<_>>();
        let mut ans = 0;

        for i in 0..n {
            for j in i+1..n {
                if masks[i] & masks[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::max_product(vec_stringify!(["abcw","baz","foo","bar","xtfn","abcdef"])), 16);
    assert_eq!(Solution::max_product(vec_stringify!(["a","ab","abc","d","cd","bcd","abcd"])), 4);
    assert_eq!(Solution::max_product(vec_stringify!(["a","aa","aaa","aaaa"])), 0);
}
