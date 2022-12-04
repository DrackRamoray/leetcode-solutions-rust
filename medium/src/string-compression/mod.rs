pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let n = chars.len();
        let mut j = n - 1;

        for i in (0..n).rev() {
            if chars[i] != chars[j] {
                if j - i > 1 {
                    let tmp = (format!("{}{}", chars[j], j - i))
                        .chars()
                        .collect::<Vec<_>>();
                    chars.splice(i + 1..=j, tmp);
                }

                j = i;
            } else if i == 0 && j - i > 0 {
                let tmp = (format!("{}{}", chars[j], j + 1 - i))
                    .chars()
                    .collect::<Vec<_>>();
                chars.splice(i..=j, tmp);
            }
        }

        chars.len() as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::compress(&mut vec!['a','a','b','b','c','c','c']), 6);
    assert_eq!(Solution::compress(&mut vec!['a']), 1);
    assert_eq!(Solution::compress(&mut vec!['a','b','b','b','b','b','b','b','b','b','b','b','b']), 4);
}
