### [46. 全排列](https://leetcode.cn/problems/permutations/)

给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。

##### 示例 1：
```
输入：nums = [1,2,3]
输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

##### 示例 2：
```
输入：nums = [0,1]
输出：[[0,1],[1,0]]
```

##### 示例 3：
```
输入：nums = [1]
输出：[[1]]
```

##### 提示：
- 1 <= nums.length <= 6
- -10 <= nums[i] <= 10
- nums 中的所有整数 互不相同

##### 题解：
```rust
impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(&mut nums, &mut ans, 0);

        ans
    }

    fn dfs(nums: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, begin: usize) {
        if begin == nums.len() {
            ans.push(nums.to_vec());
            return;
        }

        for i in begin..nums.len() {
            nums.swap(i, begin);
            Self::dfs(nums, ans, begin + 1);
            nums.swap(i, begin);
        }
    }
}
```

`回溯` `排列`
