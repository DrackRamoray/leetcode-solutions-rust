### [346. 数据流中的移动平均值](https://leetcode.cn/problems/moving-average-from-data-stream/)
给定一个整数数据流和一个窗口大小，根据该滑动窗口的大小，计算其所有整数的移动平均值。

实现 MovingAverage 类：

MovingAverage(int size) 用窗口大小 size 初始化对象。
double next(int val) 计算并返回数据流中最后 size 个值的移动平均值。


##### 示例：
```
输入：
["MovingAverage", "next", "next", "next", "next"]
[[3], [1], [10], [3], [5]]
输出：
[null, 1.0, 5.5, 4.66667, 6.0]

解释：
MovingAverage movingAverage = new MovingAverage(3);
movingAverage.next(1); // 返回 1.0 = 1 / 1
movingAverage.next(10); // 返回 5.5 = (1 + 10) / 2
movingAverage.next(3); // 返回 4.66667 = (1 + 10 + 3) / 3
movingAverage.next(5); // 返回 6.0 = (10 + 3 + 5) / 3
```

##### 提示：
- 1 <= size <= 1000
- -10<sup>5</sup> <= val <= 10<sup>5</sup>
- 最多调用 next 方法 10<sup>4</sup> 次

##### 题解：
```rust
use std::collections::VecDeque;

struct MovingAverage {
    size: usize,
    data: VecDeque<i32>,
}

impl MovingAverage {

    fn new(size: i32) -> Self {
        Self {
            size: size as usize,
            data: VecDeque::with_capacity(size as usize),
        }
    }
    
    fn next(&mut self, val: i32) -> f64 {
        if self.data.len() >= self.size {
            self.data.pop_front();
        }

        self.data.push_back(val);

        self.data.iter().map(|&x| x as f64).sum::<f64>() / (self.data.len() as f64)
    }
}

```
