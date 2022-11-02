### [254. 因子的组合](https://leetcode.cn/problems/factor-combinations/)

##### 题解：
```rust
impl Solution {
    pub fn get_factors(n: i32) -> Vec<Vec<i32>> {
        Self::dfs(n, 2)
    }

    fn dfs(n: i32, begin: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut i = begin;

        while i * i <= n {
            if n % i == 0 {
                ans.push(vec![n / i, i]);
                
                for s in Self::dfs(n / i, i).iter_mut() {
                    s.push(i);
                    ans.push(s.to_vec());
                }
            }
            i += 1;
        }

        ans
    }
}
```
