pub struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let n = s.len();

        if n < 10 {
            return vec![];
        }

        let mut cache = std::collections::HashMap::new();
        let mut ans = vec![];

        for i in 0..=n-10 {
            *cache.entry(&s[i..i+10]).or_insert(0) += 1;

            if let Some(&cnt) = cache.get(&s[i..i+10]) {
                if cnt == 2 {
                    ans.push((s[i..i+10]).to_string());
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()), vec_stringify!(vec!["AAAAACCCCC","CCCCCAAAAA"]));
        assert_eq!(Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()), vec_stringify!(vec!["AAAAAAAAAA"]));
    }
}
