### [436. 寻找右区间](https://leetcode.cn/problems/find-right-interval/)
给你一个区间数组 intervals ，其中 intervals[i] = [starti, endi] ，且每个 starti 都 不同 。

区间 i 的 右侧区间 可以记作区间 j ，并满足 startj >= endi ，且 startj 最小化 。

返回一个由每个区间 i 的 右侧区间 在 intervals 中对应下标组成的数组。如果某个区间 i 不存在对应的 右侧区间 ，则下标 i 处的值设为 -1 。


##### 示例 1：
```
输入：intervals = [[1,2]]
输出：[-1]
解释：集合中只有一个区间，所以输出-1。
```

##### 示例 2：
```
输入：intervals = [[3,4],[2,3],[1,2]]
输出：[-1,0,1]
解释：对于 [3,4] ，没有满足条件的“右侧”区间。
对于 [2,3] ，区间[3,4]具有最小的“右”起点;
对于 [1,2] ，区间[2,3]具有最小的“右”起点。
```

##### 示例 3：
```
输入：intervals = [[1,4],[2,3],[3,4]]
输出：[-1,2,-1]
解释：对于区间 [1,4] 和 [3,4] ，没有满足条件的“右侧”区间。
对于 [2,3] ，区间 [3,4] 有最小的“右”起点。
```

##### 提示：
- 1 <= intervals.length <= 2 * 10<sup>4</sup>
- intervals[i].length == 2
- -10<sup>6</sup> <= starti <= endi <= 10<sup>6</sup>
- 每个间隔的起点都 不相同

##### 题解：
```rust
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let len = intervals.len();
        let mut tmp = vec![];

        for i in 0..len {
            tmp.push((intervals[i][0], i as i32))
        }

        tmp.sort_by(|a1, a2| (a1.0).cmp(&(a2.0)));

        let mut ans = vec![0;len];

        for i in 0..len {
            let mut lo = 0;
            let mut hi = len - 1;
            let mut target = -1;

            while lo <= hi && hi < len {
                let mid = lo + (hi - lo) / 2;

                if tmp[mid].0 >= intervals[i][1] {
                    target = tmp[mid].1;
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }

            ans[i] = target;
        }

        ans
    }
}
```
