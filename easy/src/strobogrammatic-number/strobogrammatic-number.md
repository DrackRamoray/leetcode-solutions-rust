### [246. 中心对称数](https://leetcode.cn/problems/strobogrammatic-number/)

##### 题解：
```rust
impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut i = 0;
        let mut j = num.len() - 1;
        let num = num.chars().collect::<Vec<char>>();

        while i <= j && i < num.len() {
            if num[j] != Self::get_symmetry_num(num[i]) {
                return false;
            }
            i += 1;
            j -= 1;
        }

        true
    }

    fn get_symmetry_num(n: char) -> char {
        match n {
            '0' => '0',
            '1' => '1',
            '6' => '9',
            '8' => '8',
            '9' => '6',
            _ => ' ',
        }
    }
}

```
