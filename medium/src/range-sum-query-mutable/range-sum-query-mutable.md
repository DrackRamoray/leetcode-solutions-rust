### [307. 区域和检索 - 数组可修改](https://leetcode.cn/problems/range-sum-query-mutable/)
给你一个数组 nums ，请你完成两类查询。

1. 其中一类查询要求 更新 数组 nums 下标对应的值
2. 另一类查询要求返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 ，其中 left <= right

实现 NumArray 类：

- NumArray(int[] nums) 用整数数组 nums 初始化对象
- void update(int index, int val) 将 nums[index] 的值 更新 为 val
- int sumRange(int left, int right) 返回数组 nums 中索引 left 和索引 right 之间（ 包含 ）的nums元素的 和 （即，nums[left] + nums[left + 1], ..., nums[right]）


##### 示例 1：
```
输入：
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
输出：
[null, 9, null, 8]

解释：
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1,2,5]
numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8
```

##### 提示：
- 1 <= nums.length <= 3 * 10<sup>4</sup>
- -100 <= nums[i] <= 100
- 0 <= index < nums.length
- -100 <= val <= 100
- 0 <= left <= right < nums.length
- 调用 update 和 sumRange 方法次数不大于 3 * 10<sup>4</sup> 

##### 题解：
```rust
struct NumArray {
    bit_tree: BitTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut bit_tree = BitTree::new(n);
        for i in 0..n {
            bit_tree.add(i, nums[i]);
        }
        NumArray { bit_tree }
    }

    fn update(&mut self, i: i32, val: i32) {
        let i = i as usize;
        self.bit_tree.add(i as usize, val - self.bit_tree.get(i))
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        let i = i as usize;
        let j = j as usize;
        if i > 0 {
            self.bit_tree.sum(j) - self.bit_tree.sum(i - 1)
        } else {
            self.bit_tree.sum(j)
        }
    }
}

struct BitTree {
    tree: Vec<i32>,
    data: Vec<i32>,
    n: usize,
}

impl BitTree {
    fn new(n: usize) -> Self {
        let tree = vec![0; n];
        let data = vec![0; n];
        let n = n;
        BitTree { tree, data, n }
    }

    fn get(&self, i: usize) -> i32 {
        self.data[i]
    }

    fn sum(&self, i: usize) -> i32 {
        let mut res = 0;
        let down_iter = std::iter::successors(Some(i), |&i| {
            let j = i & (i + 1);
            if j > 0 {
                Some(j - 1)
            } else {
                None
            }
        });
        for j in down_iter {
            res += self.tree[j];
        }
        res
    }

    fn add(&mut self, i: usize, v: i32) {
        self.data[i] += v;
        let n = self.n;
        let up_iter = std::iter::successors(Some(i), |&i| {
            let j = i | (i + 1);
            if j < n {
                Some(j)
            } else {
                None
            }
        });
        for j in up_iter {
            self.tree[j] += v;
        }
    }
}
```
