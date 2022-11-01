### [247. 中心对称数 II](https://leetcode.cn/problems/strobogrammatic-number-ii/)

##### 题解：
```rust
impl Solution {
    pub fn find_strobogrammatic(n: i32) -> Vec<String> {
        Self::helper(n, n)
    }

    fn helper(n: i32, m: i32) -> Vec<String> {
        if n == 0 {
            return vec!["".into()];
        }

        if n == 1 {
            return vec!["0".into(), "1".into(), "8".into()];
        }

        let ans = Self::helper(n - 2, m);
        let mut res = vec![];

        for s in ans {
            if n != m {
                res.push(format!("{}{}{}", 0, s, 0));
            }

            res.push(format!("{}{}{}", 1, s, 1));
            res.push(format!("{}{}{}", 6, s, 9));
            res.push(format!("{}{}{}", 8, s, 8));
            res.push(format!("{}{}{}", 9, s, 6));
        }

        res
    }
}
```
