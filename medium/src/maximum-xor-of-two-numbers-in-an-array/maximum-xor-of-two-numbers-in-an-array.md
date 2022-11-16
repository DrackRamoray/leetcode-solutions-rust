### [421. 数组中两个数的最大异或值](https://leetcode.cn/problems/maximum-xor-of-two-numbers-in-an-array/)
给你一个整数数组 nums ，返回 nums[i] XOR nums[j] 的最大运算结果，其中 0 ≤ i ≤ j < n 。



##### 示例 1：
```
输入：nums = [3,10,5,25,2,8]
输出：28
解释：最大运算结果是 5 XOR 25 = 28.
```

##### 示例 2：
```
输入：nums = [14,70,53,83,49,91,36,80,92,51,66,70]
输出：127
```

##### 提示：
- 1 <= nums.length <= 2 * 10<sup>5</sup>
- 0 <= nums[i] <= 2<sup>31</sup> - 1

##### 题解：
```rust
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>;2],
}

impl Trie {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn insert(&mut self, num: i32) {
        let mut s = self;

        for i in (0..31).rev() {
            let index = if (num >> i) & 1 == 0 { 0 } else { 1 };
            s = s.children[index].get_or_insert_with(|| Box::new(Trie::new()));
        }
    }

    pub fn check(&self, num: i32) -> i32 {
        let mut s = self;
        let mut res = 0;

        for i in (0..31).rev() {
            if (num >> i) & 1 == 0 {
                if let Some(ref child) = s.children[1] {
                    s = child;
                    res = (res << 1) + 1;
                } else if let Some(ref child) = s.children[0] {
                    s = child;
                    res = res << 1;
                }
            } else {
                if let Some(ref child) = s.children[0] {
                    s = child;
                    res = (res << 1) + 1;
                } else if let Some(ref child) = s.children[1] {
                    s = child;
                    res = res << 1;
                }
            }
        }

        res
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut trie = Trie::new();
        for i in 1..nums.len() {
            trie.insert(nums[i - 1]);
            ans = ans.max(trie.check(nums[i]));
        }
        ans
    }
}

```
