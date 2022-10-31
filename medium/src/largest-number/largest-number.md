### [179. 最大数](https://leetcode.cn/problems/largest-number/)
给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。

注意：输出结果可能非常大，所以你需要返回一个字符串而不是整数。



##### 示例 1：
```
输入：nums = [10,2]
输出："210"
```

##### 示例 2：
```
输入：nums = [3,30,34,5,9]
输出："9534330"
```

##### 提示：
- 1 <= nums.length <= 100
- 0 <= nums[i] <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_by(|a, b| {
            let s1 = a.to_string() + &b.to_string();
            let s2 = b.to_string() + &a.to_string();

            s2.cmp(&s1)
        });

        if nums[0] == 0 {
            return "0".to_string();
        }

        nums.iter().map(|x| x.to_string()).collect::<String>()
    }
}
```
