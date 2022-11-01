use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = HashMap::new();

        for s in strings.into_iter() {
            let mut key = vec![];
            let bytes = s.as_bytes();
            let byte_1st = bytes[0];
            for byte in bytes {
                key.push((byte + 26  - byte_1st) % 26);
            }

            let entry = ans.entry(key).or_insert(vec![]);
            (*entry).push(s);
        }

        ans.into_values().collect::<Vec<Vec<String>>>()
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    let strings = vec_stringify!(vec!["abc", "bcd", "acef", "xyz", "az", "ba", "a", "z"]);
    let mut res = vec![vec_stringify!(vec!["abc", "bcd", "xyz"]), vec_stringify!(vec!["az", "ba"]), vec_stringify!(vec!["acef"]), vec_stringify!(vec!["a", "z"])];
    let mut ans = Solution::group_strings(strings);
    res.sort();
    ans.sort();
    assert_eq!(ans, res);
}
