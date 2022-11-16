pub struct Solution;

impl Solution {
    pub fn valid_word_square(words: Vec<String>) -> bool {
        let rows = words.len();

        for i in 0..rows {
            for (j, ch1) in words[i].chars().enumerate() {
                if j >= rows {
                    return false;
                }

                if let Some(ch2) = words[j].get(i..=i) {
                    if ch1.to_string() != ch2 {
                        return false;
                    }
                } else {
                    return false
                }
            }
        }

        true
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::valid_word_square(vec_stringify!(["abcd","bnrt","crmy","dtye"])), true);
    assert_eq!(Solution::valid_word_square(vec_stringify!(["abcd","bnrt","crm","dt"])), true);
    assert_eq!(Solution::valid_word_square(vec_stringify!(["ball","area","read","lady"])), false);
}
