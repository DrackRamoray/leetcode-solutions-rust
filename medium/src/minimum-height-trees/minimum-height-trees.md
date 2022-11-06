### [310. 最小高度树](https://leetcode.cn/problems/minimum-height-trees/)
树是一个无向图，其中任何两个顶点只通过一条路径连接。 换句话说，一个任何没有简单环路的连通图都是一棵树。

给你一棵包含 n 个节点的树，标记为 0 到 n - 1 。给定数字 n 和一个有 n - 1 条无向边的 edges 列表（每一个边都是一对标签），其中 edges[i] = [a<sub>i</sub>, b<sub>i</sub>] 表示树中节点 a<sub>i</sub> 和 b<sub>i</sub> 之间存在一条无向边。

可选择树中任何一个节点作为根。当选择节点 x 作为根节点时，设结果树的高度为 h 。在所有可能的树中，具有最小高度的树（即，min(h)）被称为 最小高度树 。

请你找到所有的 最小高度树 并按 任意顺序 返回它们的根节点标签列表。

树的 高度 是指根节点和叶子节点之间最长向下路径上边的数量。


##### 示例 1：
![img.png](img.png)
```
输入：n = 4, edges = [[1,0],[1,2],[1,3]]
输出：[1]
解释：如图所示，当根是标签为 1 的节点时，树的高度是 1 ，这是唯一的最小高度树。
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入：n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
输出：[3,4]
```

##### 提示：
- 1 <= n <= 2 * 10<sup>4</sup>
- edges.length == n - 1
- 0 <= ai, bi < n
- a<sub>i</sub> != b<sub>i</sub>
- 所有 (a<sub>i</sub>, b<sub>i</sub>) 互不相同
- 给定的输入 保证 是一棵树，并且 不会有重复的边

##### 题解：
```rust
use std::collections::VecDeque;

impl Solution {
    fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut graph: Vec<Vec<usize>> = vec![vec![];n as usize];
        let mut vis: Vec<bool> = vec![false;n as usize];
        let mut degree: Vec<usize> = vec![0;n as usize];

        for edge in edges {
            let i = edge[0] as usize;
            let j = edge[1] as usize;
            graph[i].push(j);
            graph[j].push(i);
            degree[i] += 1;
            degree[j] += 1;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();

        for i in 0..n as usize {
            if graph[i].len() == 1 {
                queue.push_back(i);
            }
        }

        while n > 2 {
            n -= queue.len() as i32;
            for _ in 0..queue.len() {
                if let Some(i) = queue.pop_front() {
                    vis[i] = true;
                    for &j in &graph[i] {
                        if !vis[j] {
                            degree[j] -= 1;
                            if degree[j] == 1 {
                                queue.push_back(j);
                            }
                        }
                    }
                }
            }
        }

        queue.into_iter().map(|x| x as i32).collect()
    }
}
```
