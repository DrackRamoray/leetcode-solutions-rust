### [78. 子集](https://leetcode.cn/problems/subsets/)

给你一个整数数组 nums ，数组中的元素 互不相同 。返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。你可以按 任意顺序 返回解集。



##### 示例 1：
```
输入：nums = [1,2,3]
输出：[[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

##### 示例 2：
```
输入：nums = [0]
输出：[[],[0]]
```

##### 提示：
- 1 <= nums.length <= 10
- -10 <= nums[i] <= 10
- nums 中的所有元素 互不相同

##### 题解：
```rust
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        Self::dfs(0, &mut vec![], &mut ans, &nums);

        ans
    }

    fn dfs(cur: usize, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }

        tmp.push(nums[cur]);
        Self::dfs(cur + 1, tmp, ans, nums);
        tmp.pop();
        Self::dfs(cur + 1, tmp, ans, nums);
    }
}
```

`回溯` `子集`
