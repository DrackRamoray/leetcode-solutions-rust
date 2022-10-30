### [155. 最小栈](https://leetcode.cn/problems/min-stack/)
设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

实现 MinStack 类:

- MinStack() 初始化堆栈对象。
- void push(int val) 将元素val推入堆栈。
- void pop() 删除堆栈顶部的元素。
- int top() 获取堆栈顶部的元素。
- int getMin() 获取堆栈中的最小元素。


##### 示例 1:
```
输入：
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

输出：
[null,null,null,null,-3,null,0,-2]

解释：
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin();   --> 返回 -3.
minStack.pop();
minStack.top();      --> 返回 0.
minStack.getMin();   --> 返回 -2.
```

##### 提示：
- -2<sup>31</sup> <= val <= 2<sup>31</sup> - 1
- pop、top 和 getMin 操作总是在 非空栈 上调用
- push, pop, top, and getMin最多被调用 3 * 10<sup>4</sup> 次

##### 题解：
```rust
struct MinStack {
    s: Vec<i32>,
    m: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        Self{
            s: Vec::new(),
            m: Vec::new()
        }
    }
    
    fn push(&mut self, val: i32) {
        self.s.push(val);

        if self.m.is_empty() || val < self.m[self.m.len() - 1] {
            self.m.push(val);
        } else {
            self.m.push(self.m[self.m.len() - 1]);
        }
    }
    
    fn pop(&mut self) {
        if self.s.is_empty() {
            return;
        }

        self.s.pop();
        self.m.pop();
    }
    
    fn top(&self) -> i32 {
        if self.s.is_empty() {
            return -1
        }

        self.s[self.s.len() - 1]
    }
    
    fn get_min(&self) -> i32 {
        if self.m.is_empty() {
            return -1
        }

        self.m[self.m.len() - 1]
    }
}

```
