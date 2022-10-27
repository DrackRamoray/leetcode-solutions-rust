### [47. 全排列 II](https://leetcode.cn/problems/permutations-ii/)

给定一个可包含重复数字的序列 nums ，按任意顺序 返回所有不重复的全排列。

##### 示例 1：
```
输入：nums = [1,1,2]
输出：
[[1,1,2],
 [1,2,1],
 [2,1,1]]
```

##### 示例 2：
```
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

##### 提示：
- 1 <= nums.length <= 8
- -10 <= nums[i] <= 10

##### 题解：
```rust
use std::collections::HashSet;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&mut nums, &mut ans, 0);

        ans
    }

    fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
        if begin == nums.len() {
            ans.push(nums.to_vec());
            return;
        }

        let mut dup = HashSet::new();

        for i in begin..nums.len() {
            if dup.contains(&nums[i]) {
                continue;
            }

            dup.insert(nums[i]);

            nums.swap(i, begin);

            Self::dfs(nums, ans, begin + 1);

            nums.swap(i, begin);
        }
    }
}
```

`回溯` `排列`
