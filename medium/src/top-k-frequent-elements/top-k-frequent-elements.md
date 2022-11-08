### [347. 前 K 个高频元素](https://leetcode.cn/problems/top-k-frequent-elements/)
给你一个整数数组 nums 和一个整数 k ，请你返回其中出现频率前 k 高的元素。你可以按 任意顺序 返回答案。



##### 示例 1:
```
输入: nums = [1,1,1,2,2,3], k = 2
输出: [1,2]
```

##### 示例 2:
```
输入: nums = [1], k = 1
输出: [1]
```

##### 提示：
- 1 <= nums.length <= 10<sup>5</sup>
- k 的取值范围是 [1, 数组中不相同的元素的个数]
- 题目数据保证答案唯一，换句话说，数组中前 k 个高频元素的集合是唯一的


##### 进阶：
- 你所设计算法的时间复杂度 必须 优于 O(n log n) ，其中 n 是数组大小。

##### 题解：
```rust
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
         let L = nums.len();

         if L == 0 {
             return Vec::new();
         }

        let mut m = HashMap::new();

        nums.into_iter().for_each(|x| {
            let k = m.entry(x).or_insert(0);
             *k -= 1;
        });

        let mut h = BinaryHeap::with_capacity(k as usize);

        m.into_iter().for_each(|x| {
            h.push((x.1, x.0));

             if h.len() > k as usize {
                 h.pop();
             }
        });
        
        h.into_iter().map(|x| x.1).collect::<Vec<i32>>()
    }
}
```
