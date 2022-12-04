### [435. 无重叠区间](https://leetcode.cn/problems/non-overlapping-intervals/)
给定一个区间的集合 intervals ，其中 intervals[i] = [start<sub>i</sub>, end<sub>i</sub>] 。返回 需要移除区间的最小数量，使剩余区间互不重叠 。

##### 示例 1:
```
输入: intervals = [[1,2],[2,3],[3,4],[1,3]]
输出: 1
解释: 移除 [1,3] 后，剩下的区间没有重叠。
```

##### 示例 2:
```
输入: intervals = [ [1,2], [1,2], [1,2] ]
输出: 2
解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠。
```

##### 示例 3:
```
输入: intervals = [ [1,2], [2,3] ]
输出: 0
解释: 你不需要移除任何区间，因为它们已经是无重叠的了。
```

##### 提示:
- 1 <= intervals.length <= 10<sup>5</sup>
- intervals[i].length == 2
- -5 * 10<sup>4</sup> <= start<sub>i</sub> < end<sub>i</sub> <= 5 * 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let n = intervals.len();
        let mut tmp = intervals[0][1];
        let mut ans = 0;

        for i in 1..n {
            if intervals[i][0] < tmp {
                ans += 1;
            } else {
               tmp = intervals[i][1];
            }
        }

        ans
    }
}
```
