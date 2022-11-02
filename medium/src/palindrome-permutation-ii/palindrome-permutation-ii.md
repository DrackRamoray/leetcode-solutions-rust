### [267. 回文排列 II](https://leetcode.cn/problems/palindrome-permutation-ii/)

##### 题解：
```rust
use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn generate_palindromes(s: String) -> Vec<String> {
        let mut m = HashMap::new();
        let mut vec = s.chars().collect::<Vec<char>>();

        if Self::can_permute_palindrome(&vec, &mut m) == false {
            return vec![];
        }

        let mut st = vec![];
        let mut c = None;

        for ch in m.keys() {
            if m.get(ch).unwrap() % 2 == 1 {
                c = Some(ch.to_owned());
            }
            for j in 0..(m.get(ch).unwrap() / 2) {
                st.push(ch.to_owned());
            }
        }

        let mut s = HashSet::new();

        Self::permute(&mut st, &mut s, 0, c);

        s.into_iter().collect::<Vec<String>>()
    }

    fn can_permute_palindrome(vec: &Vec<char>, m: &mut HashMap<char, i32>) -> bool {
        let mut count = 0;

        for ch in vec.iter() {
            let entry = m.entry(*ch).or_insert(0);
            *entry += 1;
            if (*entry) % 2 == 0 {
                count -= 1;
            } else {
                count += 1;
            }
        }

        count <= 1
    } 

    fn permute(vec: &mut Vec<char>, s: &mut HashSet<String>, i: usize, ch: Option<char>) {
        if i == vec.len() {
            let tmp1 = vec.iter().collect::<String>();
            let tmp2 = vec.iter().rev().collect::<String>();
            if let Some(ch) = ch {
                s.insert(format!("{}{}{}", tmp1, ch, tmp2));
            } else {
                s.insert(format!("{}{}", tmp1, tmp2));
            }
        } else {
            for j in i..vec.len() {
                if vec[j] != vec[i] || i == j {
                    vec.swap(i, j);
                    Self::permute(vec, s, i + 1, ch);
                    vec.swap(j, i);
                }
            }
        }
    }
}

```
