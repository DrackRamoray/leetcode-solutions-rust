### [40. 组合总和 II](https://leetcode.cn/problems/combination-sum-ii/)

给定一个候选人编号的集合 candidates 和一个目标数 target ，找出 candidates 中所有可以使数字和为 target 的组合。

candidates 中的每个数字在每个组合中只能使用 一次 。

注意：解集不能包含重复的组合。

##### 示例 1:
```
输入: candidates = [10,1,2,7,6,1,5], target = 8,
输出:
[
[1,1,6],
[1,2,5],
[1,7],
[2,6]
]
```

##### 示例 2:
```
输入: candidates = [2,5,2,1,2], target = 5,
输出:
[
[1,2,2],
[5]
]
```

##### 提示:
- 1 <= candidates.length <= 100
- 1 <= candidates[i] <= 50
- 1 <= target <= 30

##### 题解：
```rust
impl Solution {
    fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        let n = candidates.len();
        candidates.sort();

        Self::dfs(&candidates, &mut Vec::new(), &mut ans, target, 0, n);

        ans
    }

    fn dfs(candidates: &[i32], selected: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, target: i32, cur: usize, n: usize) {
        if target == 0 {
            ans.push(selected.to_vec());
            return;
        }

        if target > 0 {
            for i in cur..n {
                if i > cur && candidates[i] == candidates[i - 1] {
                    continue;
                }
                
                selected.push(candidates[i]);
                Self::dfs(candidates, selected, ans, target - candidates[i], i + 1, n);
                selected.pop();
            }
        }
    }
}
```

`回溯` `组合`
