### [90. 子集 II](https://leetcode.cn/problems/subsets-ii/)

给你一个整数数组 nums ，其中可能包含重复元素，请你返回该数组所有可能的子集（幂集）。

解集 不能 包含重复的子集。返回的解集中，子集可以按 任意顺序 排列。



##### 示例 1：
```
输入：nums = [1,2,2]
输出：[[],[1],[1,2],[1,2,2],[2],[2,2]]
```

##### 示例 2：
```
输入：nums = [0]
输出：[[],[0]]
```

##### 提示：
- 1 <= nums.length <= 10
- -10 <= nums[i] <= 10

##### 题解：
```rust
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];

        nums.sort();
        
        Self::dfs(&nums, &mut ans, &mut vec![], 0, false);

        ans
    }

    fn dfs(nums: &Vec<i32>, ans: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>, cur: usize, choose_pre: bool) {
        if cur == nums.len() {
            ans.push(tmp.to_vec());
            return;
        }

        Self::dfs(nums, ans, tmp, cur + 1, false);

        if !choose_pre && cur > 0 && nums[cur-1] == nums[cur] {
            return;
        }

        tmp.push(nums[cur]);

        Self::dfs(nums, ans, tmp, cur + 1, true);

        tmp.pop();
    }
}
```
`回溯` `子集`
