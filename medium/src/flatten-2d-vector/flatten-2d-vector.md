### [251. 展开二维向量](https://leetcode.cn/problems/flatten-2d-vector/)

##### 题解：
```rust
struct Vector2D {
    pc: usize,
    data: Vec<i32>,
}

impl Vector2D {

    fn new(vec: Vec<Vec<i32>>) -> Self {
        Self {
            pc: 0,
            data: vec.into_iter().flatten().collect(),
        }
    }
    
    fn next(&mut self) -> i32 {
        if self.pc >= self.data.len() {
            return self.data[self.data.len() - 1];
        }

        let v = self.data[self.pc];

        self.pc += 1;

        v
    }
    
    fn has_next(&mut self) -> bool {
        self.pc < self.data.len()
    }
}

```
