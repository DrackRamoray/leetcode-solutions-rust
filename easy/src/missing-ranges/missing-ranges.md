### [163. 缺失的区间](https://leetcode.cn/problems/missing-ranges/)

##### 题解：
```rust
impl Solution {
    pub fn find_missing_ranges(mut nums: Vec<i32>, mut lower: i32, upper: i32) -> Vec<String> {
        nums.push(upper + 1);
        lower -= 1;

        nums.iter().fold(Vec::new(), |mut acc, &cur| {
            let diff = cur - lower;

            if diff == 2 {
                acc.push((lower + 1).to_string());
            } else if diff > 2 {
                acc.push((lower + 1).to_string() + "->" + &(cur - 1).to_string());
            }

            lower = cur;

            acc
        })
    }
}
```
