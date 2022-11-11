### [399. 除法求值](https://leetcode.cn/problems/evaluate-division/)
给你一个变量对数组 equations 和一个实数值数组 values 作为已知条件，其中 equations[i] = [A<sub>i</sub>, B<sub>i</sub>] 和 values[i] 共同表示等式 Ai / Bi = values[i] 。每个 Ai 或 Bi 是一个表示单个变量的字符串。

另有一些以数组 queries 表示的问题，其中 queries[j] = [C<sub>j</sub>, D<sub>j</sub>] 表示第 j 个问题，请你根据已知条件找出 C<sub>j</sub> / D<sub>j</sub> = ? 的结果作为答案。

返回 所有问题的答案 。如果存在某个无法确定的答案，则用 -1.0 替代这个答案。如果问题中出现了给定的已知条件中没有出现的字符串，也需要用 -1.0 替代这个答案。

注意：输入总是有效的。你可以假设除法运算中不会出现除数为 0 的情况，且不存在任何矛盾的结果。



##### 示例 1：
```
输入：equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
输出：[6.00000,0.50000,-1.00000,1.00000,-1.00000]
解释：
条件：a / b = 2.0, b / c = 3.0
问题：a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ?
结果：[6.0, 0.5, -1.0, 1.0, -1.0 ]
```

##### 示例 2：
```
输入：equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
输出：[3.75000,0.40000,5.00000,0.20000]
```

##### 示例 3：
```
输入：equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
输出：[0.50000,2.00000,-1.00000,-1.00000]
```

##### 提示：
- 1 <= equations.length <= 20
- equations[i].length == 2
- 1 <= Ai.length, Bi.length <= 5
- values.length == equations.length
- 0.0 < values[i] <= 20.0
- 1 <= queries.length <= 20
- queries[i].length == 2
- 1 <= Cj.length, Dj.length <= 5
- A<sub>i</sub>, B<sub>i</sub>, C<sub>j</sub>, D<sub>j</sub> 由小写英文字母与数字组成

##### 题解：
```rust
struct UnionFind {
    parents: Vec<usize>,
    weights: Vec<f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parents = vec![0;n];
        let mut weights = vec![0.0;n];

        for i in 0..n {
            parents[i] = i;
            weights[i] = 1.0;
        }

        Self {
            parents,
            weights,
        }
    }

    fn uion(&mut self, i: usize, j: usize, weight: f64) {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            return;
        }

        self.parents[x] = y;
        self.weights[x] = self.weights[j] * weight / self.weights[i];
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parents[i] {
            let x = self.parents[i];
            self.parents[i] = self.find(self.parents[i]);
            self.weights[i] *= self.weights[x];
        }

        return self.parents[i];
    }

    fn is_connected(&mut self, i: usize, j: usize) -> f64 {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            self.weights[i] / self.weights[j]
        } else {
            -1.0
        }
    }
}

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let n = equations.len();
        let mut un = UnionFind::new(2 * n);
        let mut mp = std::collections::HashMap::new();
        let mut id = 0;

        for i in 0..n {
            let equation = &equations[i];
            let v1 = &equation[0];
            let v2 = &equation[1];

            if !mp.contains_key(&v1) {
                mp.insert(v1, id);
                id += 1;
            }

            if !mp.contains_key(&v2) {
                mp.insert(v2, id);
                id += 1;
            }

            if let (Some(&u), Some(&v)) = (mp.get(&v1), mp.get(&v2)) {
                un.uion(u, v, values[i]);
            }
        }

        let m = queries.len();
        let mut ans = vec![-1.0;m];

        for i in 0..m {
            let query = &queries[i];
            let v1 = &query[0];
            let v2 = &query[1];

            if let (Some(&u), Some(&v)) = (mp.get(&v1), mp.get(&v2)) {
                ans[i] = un.is_connected(u, v);
            }
        }

        ans
    }
}
```
