### [398. 随机数索引](https://leetcode.cn/problems/random-pick-index/)
给你一个可能含有 重复元素 的整数数组 nums ，请你随机输出给定的目标数字 target 的索引。你可以假设给定的数字一定存在于数组中。

实现 Solution 类：

- Solution(int[] nums) 用数组 nums 初始化对象。
- int pick(int target) 从 nums 中选出一个满足 nums[i] == target 的随机索引 i 。如果存在多个有效的索引，则每个索引的返回概率应当相等。


##### 示例：
```
输入
["Solution", "pick", "pick", "pick"]
[[[1, 2, 3, 3, 3]], [3], [1], [3]]
输出
[null, 4, 0, 2]

解释
Solution solution = new Solution([1, 2, 3, 3, 3]);
solution.pick(3); // 随机返回索引 2, 3 或者 4 之一。每个索引的返回概率应该相等。
solution.pick(1); // 返回 0 。因为只有 nums[0] 等于 1 。
solution.pick(3); // 随机返回索引 2, 3 或者 4 之一。每个索引的返回概率应该相等。
```

##### 提示：
- 1 <= nums.length <= 2 * 10<sup>4</sup>
- -2<sup>31</sup> <= nums[i] <= 2<sup>31</sup> - 1
- target 是 nums 中的一个整数
- 最多调用 pick 函数 10<sup>4</sup> 次

##### 题解：
```rust
use std::collections::HashMap;
use rand::{thread_rng, Rng};

struct Solution {
    data: HashMap<i32, Vec<usize>>
}

impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        let mut data = HashMap::new();

        for i in 0..nums.len() {
            data.entry(nums[i]).or_insert(vec![]).push(i);
        }

        Self {
            data,
        }
    }
    
    fn pick(&self, target: i32) -> i32 {
        if let Some(vals) = self.data.get(&target) {
            let mut rng = thread_rng();
            vals[rng.gen_range(0, vals.len())] as i32
        } else {
            -1
        }
    }
}

```
