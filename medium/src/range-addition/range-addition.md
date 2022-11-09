### [370. 区间加法](https://leetcode.cn/problems/range-addition/)
假设你有一个长度为 n 的数组，初始情况下所有的数字均为 0，你将会被给出 k​​​​​​​ 个更新的操作。

其中，每个操作会被表示为一个三元组：[startIndex, endIndex, inc]，你需要将子数组 A[startIndex ... endIndex]（包括 startIndex 和 endIndex）增加 inc。

请你返回 k 次操作后的数组。

##### 示例:
```
输入: length = 5, updates = [[1,3,2],[2,4,3],[0,2,-2]]
输出: [-2,0,3,5,3]
```
> **解释:**
> ```
> 初始状态:
> [0,0,0,0,0]
> 
> 进行了操作 [1,3,2] 后的状态:
> [0,2,2,2,0]
>
> 进行了操作 [2,4,3] 后的状态:
> [0,2,5,5,3]
>
> 进行了操作 [0,2,-2] 后的状态:
> [-2,0,3,5,3]
> ```

##### 题解：
```rust
impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let len = length as usize;
        let mut vec = vec![0;len];

        for update in updates {
            vec[update[0] as usize] += update[2];

            if update[1] as usize + 1 < len {
                vec[update[1] as usize + 1] -= update[2];
            }
        }

        for i in 1..len {
            vec[i] += vec[i-1];
        }

        vec
    }
}
```
