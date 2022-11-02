### [256. 粉刷房子](https://leetcode.cn/problems/paint-house/)

##### 题解：
```rust
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let len = costs.len();

        let mut r = costs[0][0];
        let mut g = costs[0][1];
        let mut b = costs[0][2];

        for i in 1..len {
            let r1 = g.min(b) + costs[i][0];
            let g1 = r.min(b) + costs[i][1];
            b = r.min(g) + costs[i][2];
            r = r1;
            g = g1;
        }

       r.min(g).min(b)
    }
}
```
