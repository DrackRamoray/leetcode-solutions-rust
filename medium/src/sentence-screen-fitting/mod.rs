pub struct Solution;

impl Solution {
    pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        let paragraph = sentence.into_iter().map(|s| s + " ").collect::<String>();
        let byts = paragraph.as_bytes();
        let n = byts.len();

        let mut j = 0;

        for _ in 0..rows as usize {
            j += cols;

            if byts[j as usize % n] == b' ' {
                j += 1;
            } else {
                while j > 0 && byts[(j as usize - 1) % n] != b' ' {
                    j -= 1;
                }
            }
        }

        j / n as i32
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;
    assert_eq!(Solution::words_typing(vec_stringify!(["hello", "world"]), 2, 8), 1);
    assert_eq!(Solution::words_typing(vec_stringify!(["a", "bcd", "e"]), 3, 6), 2);
    assert_eq!(Solution::words_typing(vec_stringify!(["I", "had", "apple", "pie"]), 4, 5), 1);
}
