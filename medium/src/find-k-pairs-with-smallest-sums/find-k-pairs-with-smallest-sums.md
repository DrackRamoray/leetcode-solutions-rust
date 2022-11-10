### [373. 查找和最小的 K 对数字](https://leetcode.cn/problems/find-k-pairs-with-smallest-sums/)
给定两个以 升序排列 的整数数组 nums1 和 nums2 , 以及一个整数 k 。

定义一对值 (u,v)，其中第一个元素来自 nums1，第二个元素来自 nums2 。

请找到和最小的 k 个数对 (u<sub>1</sub>,v<sub>1</sub>),  (u<sub>2</sub>,v<sub>2</sub>)  ...  (u<sub>k</sub>,v<sub>k</sub>) 。



##### 示例 1:
```
输入: nums1 = [1,7,11], nums2 = [2,4,6], k = 3
输出: [1,2],[1,4],[1,6]
解释: 返回序列中的前 3 对数：
[1,2],[1,4],[1,6],[7,2],[7,4],[11,2],[7,6],[11,4],[11,6]
```

##### 示例 2:
```
输入: nums1 = [1,1,2], nums2 = [1,2,3], k = 2
输出: [1,1],[1,1]
解释: 返回序列中的前 2 对数：
[1,1],[1,1],[1,2],[2,1],[1,2],[2,2],[1,3],[1,3],[2,3]
```

##### 示例 3:
```
输入: nums1 = [1,2], nums2 = [3], k = 3
输出: [1,3],[2,3]
解释: 也可能序列中所有的数对都被返回:[1,3],[2,3]
```

##### 提示:
- 1 <= nums1.length, nums2.length <= 10<sup>5</sup>
- -10<sup>9</sup> <= nums1[i], nums2[i] <= 10<sup>9</sup>
- nums1 和 nums2 均为升序排列
- 1 <= k <= 1000

##### 题解：
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let k = k as usize;

        for i in 0..(k.min(nums1.len())) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0)));
        }

        let mut ans = vec![];

        for i in (1..=k).rev() {
            if let Some(Reverse((sum, j, k))) = heap.pop() {
                ans.push(vec![nums1[j], nums2[k]]);

                if k < nums2.len() - 1 {
                    heap.push(Reverse((nums1[j] + nums2[k+1], j, k + 1)));
                }
            }
        }

        ans
    }
}
```
