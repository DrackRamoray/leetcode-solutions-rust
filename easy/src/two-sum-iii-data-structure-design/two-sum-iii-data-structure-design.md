### [170. 两数之和 III - 数据结构设计](https://leetcode.cn/problems/two-sum-iii-data-structure-design/)

##### 题解：
```rust
use std::collections::HashMap;

struct TwoSum {
    cache: HashMap<i32, i32>,
}

impl TwoSum {

    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
    
    fn add(&mut self, number: i32) {
        let counter = self.cache.entry(number).or_insert(0);
        *counter += 1;
    }

    fn find(&self, value: i32) -> bool {
        for num in self.cache.keys() {
            if let Some(v) = self.cache.get(&(value - num)) {
                if *num != value - num || v > &1 {
                    return true;
                }
            }
        }

        false
    }
}
```
