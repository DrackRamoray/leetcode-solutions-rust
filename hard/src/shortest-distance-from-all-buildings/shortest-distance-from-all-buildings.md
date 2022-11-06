### [317. 离建筑物最近的距离](https://leetcode.cn/problems/shortest-distance-from-all-buildings/)
给你一个 m × n 的网格，值为 0 、 1 或 2 ，其中:

- 每一个 0 代表一块你可以自由通过的 空地
- 每一个 1 代表一个你不能通过的 建筑
- 每个 2 标记一个你不能通过的 障碍

你想要在一块空地上建造一所房子，在 最短的总旅行距离 内到达所有的建筑。你只能上下左右移动。

返回到该房子的 最短旅行距离 。如果根据上述规则无法建造这样的房子，则返回 -1 。

总旅行距离 是朋友们家到聚会地点的距离之和。

使用 曼哈顿距离 计算距离，其中距离 (p1, p2) = |p2.x - p1.x | + | p2.y - p1.y | 。



##### 示例  1：
![img.png](img.png)
```
输入：grid = [[1,0,2,0,1],[0,0,0,0,0],[0,0,1,0,0]]
输出：7
解析：给定三个建筑物 (0,0)、(0,4) 和 (2,2) 以及一个位于 (0,2) 的障碍物。
由于总距离之和 3+3+1=7 最优，所以位置 (1,2) 是符合要求的最优地点。
故返回7。
```

##### 示例 2:
```
输入: grid = [[1,0]]
输出: 1
```

##### 示例 3:
```
输入: grid = [[1]]
输出: -1
```

##### 提示:
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 50
- grid[i][j] 是 0, 1 或 2
- grid 中 至少 有 一幢 建筑

##### 题解：
```rust
impl Solution {
    pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = i32::MAX;
        let mut val = 0;
        let mut sum = grid.clone();
        let mut dist = grid.clone();
        let dirs = [(0,-1),(-1,0),(0,1),(1,0)];
        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    ans = i32::MAX;
                    queue.push_back((i, j));

                    while let Some((x, y)) = queue.pop_front() {
                        for &(dx, dy) in &dirs {
                            let new_x = (x as i32 + dx) as usize;
                            let new_y = (y as i32 + dy) as usize;

                            if new_x < n && new_y < m && grid[new_x][new_y] == val {
                                grid[new_x][new_y] -= 1;
                                dist[new_x][new_y] = dist[x][y] + 1;
                                sum[new_x][new_y] += dist[new_x][new_y] - 1;
                                queue.push_back((new_x, new_y));
                                ans = ans.min(sum[new_x][new_y]);
                            }
                        }
                    }

                    val -= 1;
                }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}
```
