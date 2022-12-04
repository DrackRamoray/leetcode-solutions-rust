### [444. 序列重建](https://leetcode.cn/problems/sequence-reconstruction/)
给定一个长度为 n 的整数数组 nums ，其中 nums 是范围为 [1，n] 的整数的排列。还提供了一个 2D 整数数组 sequences ，其中 sequences[i] 是 nums 的子序列。
检查 nums 是否是唯一的最短 超序列 。最短 超序列 是 长度最短 的序列，并且所有序列 sequences[i] 都是它的子序列。对于给定的数组 sequences ，可能存在多个有效的 超序列 。

- 例如，对于 sequences = [[1,2],[1,3]] ，有两个最短的 超序列 ，[1,2,3] 和 [1,3,2] 。
- 而对于 sequences = [[1,2],[1,3],[1,2,3]] ，唯一可能的最短 超序列 是 [1,2,3] 。[1,2,3,4] 是可能的超序列，但不是最短的。

如果 nums 是序列的唯一最短 超序列 ，则返回 true ，否则返回 false 。

子序列 是一个可以通过从另一个序列中删除一些元素或不删除任何元素，而不改变其余元素的顺序的序列。



##### 示例 1：
```
输入：nums = [1,2,3], sequences = [[1,2],[1,3]]
输出：false
解释：有两种可能的超序列：[1,2,3]和[1,3,2]。
序列 [1,2] 是[1,2,3]和[1,3,2]的子序列。
序列 [1,3] 是[1,2,3]和[1,3,2]的子序列。
因为 nums 不是唯一最短的超序列，所以返回false。
```

##### 示例 2：
```
输入：nums = [1,2,3], sequences = [[1,2]]
输出：false
解释：最短可能的超序列为 [1,2]。
序列 [1,2] 是它的子序列：[1,2]。
因为 nums 不是最短的超序列，所以返回false。
```

##### 示例 3：
```
输入：nums = [1,2,3], sequences = [[1,2],[1,3],[2,3]]
输出：true
解释：最短可能的超序列为[1,2,3]。
序列 [1,2] 是它的一个子序列：[1,2,3]。
序列 [1,3] 是它的一个子序列：[1,2,3]。
序列 [2,3] 是它的一个子序列：[1,2,3]。
因为 nums 是唯一最短的超序列，所以返回true。
```

##### 提示：
- n == nums.length
- 1 <= n <= 10<sup>4</sup>
- nums 是 [1, n] 范围内所有整数的排列
- 1 <= sequences.length <= 10<sup>4</sup>
- 1 <= sequences[i].length <= 10<sup>4</sup>
- 1 <= sum(sequences[i].length) <= 10<sup>5</sup>
- 1 <= sequences[i][j] <= n
- sequences 的所有数组都是 唯一 的
- sequences[i] 是 nums 的一个子序列

##### 题解：
```rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let mut n = nums.len();
        let mut indegree = vec![0;n];
        let mut vis = vec![false;n];
        let mut graph = vec![HashSet::new();n];

        for seq in sequences {
            let m = seq.len();

            for i in 0..m {
                let j = seq[i] as usize - 1;

                if j >= n {
                    return false;
                }

                vis[j] = true;

                if i + 1 < m {
                    let k = seq[i+1] as usize - 1;

                    if k >= n || k == j {
                        return false;
                    }

                    if graph[j].insert(k) {
                        indegree[k] += 1;
                    }
                }
            }
        }

        if vis.iter().any(|&v| v == false) {
            return false;
        }

        let mut queue = VecDeque::new();
        let mut stack = vec![];

        for i in 0..n {
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }

        while let Some(i) = queue.pop_front() {
            if queue.len() > 0 {
                return false;
            }

            stack.push(i as i32 + 1);

            for &j in graph[i].iter() {
                indegree[j] -= 1;

                if indegree[j] == 0 {
                    queue.push_back(j);
                }
            }
        }

        stack == nums
    }
}
```
