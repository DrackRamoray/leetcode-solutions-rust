### [239. 滑动窗口最大值](https://leetcode.cn/problems/sliding-window-maximum/)
给你一个整数数组 nums，有一个大小为 k 的滑动窗口从数组的最左侧移动到数组的最右侧。你只可以看到在滑动窗口内的 k 个数字。滑动窗口每次只向右移动一位。

返回 滑动窗口中的最大值 。



##### 示例 1：
```
输入：nums = [1,3,-1,-3,5,3,6,7], k = 3
输出：[3,3,5,5,6,7]
解释：
滑动窗口的位置                最大值
---------------               -----
[1  3  -1] -3  5  3  6  7       3
1 [3  -1  -3] 5  3  6  7       3
1  3 [-1  -3  5] 3  6  7       5
1  3  -1 [-3  5  3] 6  7       5
1  3  -1  -3 [5  3  6] 7       6
1  3  -1  -3  5 [3  6  7]      7
```

##### 示例 2：
```
输入：nums = [1], k = 1
输出：[1]
```

##### 提示：
- 1 <= nums.length <= 10<sup>5</sup>
- -10<sup>4</sup> <= nums[i] <= 10<sup>4</sup>
- 1 <= k <= nums.length

##### 题解：
```rust
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut n = nums.len();
        let k = k as usize;
        let mut queue = std::collections::VecDeque::new();

        for i in 0..k {
            while queue.len() > 0 && nums[i] > nums[queue[queue.len() - 1]] {
                queue.pop_back();
            }
            queue.push_back(i);
        }

        let mut ans = vec![nums[queue[0]]];

        for i in k..n {
            while queue.len() > 0 && nums[i] > nums[queue[queue.len() - 1]] {
                queue.pop_back();
            }
            queue.push_back(i);
            while queue[0] <= i - k {
                queue.pop_front();
            }
            ans.push(nums[queue[0]])
        }

        ans
    }
}

```
