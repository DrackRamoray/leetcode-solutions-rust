### [295. 数据流的中位数](https://leetcode.cn/problems/find-median-from-data-stream/)
中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。

例如，

[2,3,4] 的中位数是 3

[2,3] 的中位数是 (2 + 3) / 2 = 2.5

设计一个支持以下两种操作的数据结构：

- void addNum(int num) - 从数据流中添加一个整数到数据结构中。
- double findMedian() - 返回目前所有元素的中位数。

##### 示例：
```
addNum(1)
addNum(2)
findMedian() -> 1.5
addNum(3)
findMedian() -> 2
```

##### 进阶:
- 如果数据流中所有整数都在 0 到 100 范围内，你将如何优化你的算法？
- 如果数据流中 99% 的整数都在 0 到 100 范围内，你将如何优化你的算法？

##### 题解：
```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct MedianFinder {
    min_queue: BinaryHeap<i32>,
    max_queue: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {

    fn new() -> Self {
        Self {
            min_queue: BinaryHeap::new(),
            max_queue: BinaryHeap::new(),
        }
    }
    
    fn add_num(&mut self, num: i32) {
        match self.min_queue.peek() {
            Some(&v) => match v >= num {
                true => {
                    self.min_queue.push(num);

                    if self.min_queue.len() > self.max_queue.len() + 1 {
                        if let Some(u) = self.min_queue.pop() {
                            self.max_queue.push(Reverse(u));
                        }
                    }
                }
                false => {
                    self.max_queue.push(Reverse(num));

                    if self.max_queue.len() > self.min_queue.len() {
                        if let Some(Reverse(u)) = self.max_queue.pop() {
                            self.min_queue.push(u);
                        }
                    }
                }
            },
            None => {
                self.min_queue.push(num);
            }
        }
    }
    
    fn find_median(&self) -> f64 {
        if self.min_queue.len() > self.max_queue.len() {
            if let Some(&v) = self.min_queue.peek() {
                return v as f64;
            }
        }

        if let (Some(&v), Some(&Reverse(u))) = (self.min_queue.peek(), self.max_queue.peek()) {
            return (v as f64 + u as f64) / 2.0;
        }

        -1.0
    }
}

```
