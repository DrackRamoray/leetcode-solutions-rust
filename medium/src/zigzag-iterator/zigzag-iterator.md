### [281. 锯齿迭代器](https://leetcode.cn/problems/zigzag-iterator/)
给出两个一维的向量，请你实现一个迭代器，交替返回它们中间的元素。

##### 示例:
```
输入:
v1 = [1,2]
v2 = [3,4,5,6]

输出: [1,3,2,4,5,6]

解析: 通过连续调用 next 函数直到 hasNext 函数返回 false，
next 函数返回值的次序应依次为: [1,3,2,4,5,6]。
```

##### 拓展：
- 假如给你 k 个一维向量呢？你的代码在这种情况下的扩展性又会如何呢?

##### 拓展声明：
- “锯齿” 顺序对于 k > 2 的情况定义可能会有些歧义。所以，假如你觉得 “锯齿” 这个表述不妥，也可以认为这是一种 “循环”。例如：

```
输入:
[1,2,3]
[4,5,6,7]
[8,9]

输出: [1,4,8,2,5,9,3,6,7].
```

##### 题解：
```rust
use std::collections::VecDeque;

struct ZigzagIterator {
    data: VecDeque<i32>
}

impl ZigzagIterator {
    fn new(v1: Vec<i32>, v2: Vec<i32>) -> Self {
        let mut data = VecDeque::new();
        let n = v1.len();
        let m = v2.len();
        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            data.push_back(v1[i]);
            data.push_back(v2[j]);
            i += 1;
            j += 1;
        }

        while i < n {
            data.push_back(v1[i]);
            i += 1;
        }

        while j < m {
            data.push_back(v2[j]);
            j += 1;
        }

        Self {
            data,
        }
    }
    
    fn next(&mut self) -> i32 {
        if let Some(v) = self.data.pop_front() {
            v
        } else {
            -1
        }
    }
    
    fn has_next(&self) -> bool {
        self.data.len() > 0
    }
}

```
