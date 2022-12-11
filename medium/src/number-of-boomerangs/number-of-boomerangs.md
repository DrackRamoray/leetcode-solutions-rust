### [447. 回旋镖的数量](https://leetcode.cn/problems/number-of-boomerangs/)
给定平面上 n 对 互不相同 的点 points ，其中 points[i] = [x<sub>i</sub>, y<sub>i</sub>] 。回旋镖 是由点 (i, j, k) 表示的元组 ，其中 i 和 j 之间的距离和 i 和 k 之间的欧式距离相等（需要考虑元组的顺序）。

返回平面上所有回旋镖的数量。


##### 示例 1：
```
输入：points = [[0,0],[1,0],[2,0]]
输出：2
解释：两个回旋镖为 [[1,0],[0,0],[2,0]] 和 [[1,0],[2,0],[0,0]]
```

##### 示例 2：
```
输入：points = [[1,1],[2,2],[3,3]]
输出：2
```

##### 示例 3：
```
输入：points = [[1,1]]
输出：0
```

##### 提示：
- n == points.length
- 1 <= n <= 500
- points[i].length == 2
- -10<sup>4</sup> <= x<sub>i</sub>, y<sub>i</sub> <= 10<sup>4</sup>
- 所有点都 互不相同

##### 题解：
```rust
macro_rules! dist {
    ($a:expr, $b:expr) => {
        ($a[0] - $b[0]).pow(2) + ($a[1] - $b[1]).pow(2)
    }
}

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut mp = std::collections::HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let distance = dist!(points[i], points[j]);
                    *mp.entry(distance).or_insert(0) += 1;
                }
            }

            for &v in mp.values() {
                if v > 1 {
                    ans += v * (v - 1);
                }
            }

            mp.clear();
        }

        ans
    }
}
```
