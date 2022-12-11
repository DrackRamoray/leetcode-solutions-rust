pub struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let cache = s.chars().fold(std::collections::HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1;
            acc
        });
        let mut vec = cache.into_iter().map(|(c, t)| (t, c)).collect::<Vec<(usize, char)>>();
        vec.sort_by(|a, b| b.cmp(a));
        vec.iter().fold(String::new(), |mut acc, (t, c)| {
            acc.push_str(&c.to_string().repeat(*t));
            acc
        })
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::frequency_sort("tree".to_string()), "eetr".to_string());
    assert_eq!(Solution::frequency_sort("cccaaa".to_string()), "cccaaa".to_string());
    assert_eq!(Solution::frequency_sort("Aabb".to_string()), "bbaA".to_string());
}
