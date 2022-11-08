### [349. 两个数组的交集](https://leetcode.cn/problems/intersection-of-two-arrays/)
给定两个数组 nums1 和 nums2 ，返回 它们的交集 。输出结果中的每个元素一定是 唯一 的。我们可以 不考虑输出结果的顺序 。



##### 示例 1：
```
输入：nums1 = [1,2,2,1], nums2 = [2,2]
输出：[2]
```

##### 示例 2：
```
输入：nums1 = [4,9,5], nums2 = [9,4,9,8,4]
输出：[9,4]
解释：[4,9] 也是可通过的
```

##### 提示：
- 1 <= nums1.length, nums2.length <= 1000
- 0 <= nums1[i], nums2[i] <= 1000

##### 题解：
```rust
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<HashSet<i32>>()
            .intersection(&nums2
                .into_iter()
                .collect::<HashSet<i32>>()
            )
            .cloned()
            .collect()
    }
}
```
