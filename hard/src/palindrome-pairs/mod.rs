pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mp = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect::<HashMap<_, _>>();

        let mut ans = Vec::new();

        for (i, word) in words.iter().enumerate() {
            for k in 0..word.len() {
                if (0..=k / 2).all(|l| word[l..=l] == word[k-l..=k-l]) {
                    if let Some(&j) = mp.get(&(word[k+1..].chars().rev().collect::<String>())) {
                        ans.push(vec![j as i32, i as i32]);
                    }
                }
            }

            let rev = word.chars().rev().collect::<String>();
            if let Some(&j) = mp.get(&rev) {
                if i != j {
                    ans.push(vec![i as i32, j as i32]);
                }
            }

            for k in 0..rev.len() {
                if (0..=k/2).all(|l| rev[l..=l] == rev[k-l..=k-l]) {
                    if let Some(&j) = mp.get(&rev[k+1..]) {
                        ans.push(vec![i as i32, j as i32]);
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::palindrome_pairs(vec_stringify!(["abcd","dcba","lls","s","sssll"])), vec![vec![0,1],vec![1,0],vec![3,2],vec![2,4]]);
    assert_eq!(Solution::palindrome_pairs(vec_stringify!(["bat","tab","cat"])), vec![vec![0,1],vec![1,0]] );
    assert_eq!(Solution::palindrome_pairs(vec_stringify!(["a",""])), vec![vec![1,0],vec![0,1]]);
}
