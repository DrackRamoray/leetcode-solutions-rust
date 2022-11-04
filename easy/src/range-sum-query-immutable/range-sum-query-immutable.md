### [303. 区域和检索 - 数组不可变](https://leetcode.cn/problems/range-sum-query-immutable/)
给定一个整数数组  nums，处理以下类型的多个查询:

计算索引 left 和 right （包含 left 和 right）之间的 nums 元素的 和 ，其中 left <= right
实现 NumArray 类：

NumArray(int[] nums) 使用数组 nums 初始化对象
int sumRange(int i, int j) 返回数组 nums 中索引 left 和 right 之间的元素的 总和 ，包含 left 和 right 两点（也就是 nums[left] + nums[left + 1] + ... + nums[right] )


##### 示例 1：
```
输入：
["NumArray", "sumRange", "sumRange", "sumRange"]
[[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
输出：
[null, 1, -1, -3]

解释：
NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1))
numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
```

##### 提示：
- 1 <= nums.length <= 10<sup>4</sup>
- -10<sup>5</sup> <= nums[i] <= 10<sup>5</sup>
- 0 <= i <= j < nums.length
- 最多调用 10<sup>4</sup> 次 sumRange 方法

##### 题解：
```rust
struct NumArray {
    s: Vec<i32>
}

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut s = vec![0;n+1];

        for i in 0..n {
            s[i+1] = s[i] + nums[i]
        }

        NumArray{s}
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.s[(right + 1) as usize] - self.s[left as usize]
    }
}
```
