### [252. 会议室](https://leetcode.cn/problems/meeting-rooms/)

##### 题解：
```rust
impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut y = 0;

        for x in intervals.iter() {
            if y > x[0] {
                return false;
            }

            y = x[1];
        }

        true
    }
}

```
