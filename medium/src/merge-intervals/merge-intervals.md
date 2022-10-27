### [56. 合并区间](https://leetcode.cn/problems/merge-intervals/)

以数组 intervals 表示若干个区间的集合，其中单个区间为 intervals[i] = [starti, endi] 。请你合并所有重叠的区间，并返回 一个不重叠的区间数组，该数组需恰好覆盖输入中的所有区间 。

##### 示例 1：
```
输入：intervals = [[1,3],[2,6],[8,10],[15,18]]
输出：[[1,6],[8,10],[15,18]]
解释：区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
```

##### 示例 2：
```
输入：intervals = [[1,4],[4,5]]
输出：[[1,5]]
解释：区间 [1,4] 和 [4,5] 可被视为重叠区间。
```

##### 提示：
- 1 <= intervals.length <= 10<sup>4</sup>
- intervals[i].length == 2
- 0 <= starti <= endi <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = vec![intervals[0].clone()];

        let n = intervals.len();

        for i in 1..n {
            let m = ans.len() - 1;
            if intervals[i][0] <= ans[m][1] {
                if intervals[i][1] > ans[m][1] {
                    ans[m][1] = intervals[i][1];
                }
            } else {
                ans.push(intervals[i].clone());
            }
        }

        ans
    }
}
```

`排序`
