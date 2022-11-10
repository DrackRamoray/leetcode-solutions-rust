### [381. O(1) 时间插入、删除和获取随机元素 - 允许重复](https://leetcode.cn/problems/insert-delete-getrandom-o1-duplicates-allowed/)
RandomizedCollection 是一种包含数字集合(可能是重复的)的数据结构。它应该支持插入和删除特定元素，以及删除随机元素。

实现 RandomizedCollection 类:

- RandomizedCollection()初始化空的 RandomizedCollection 对象。
- bool insert(int val) 将一个 val 项插入到集合中，即使该项已经存在。如果该项不存在，则返回 true ，否则返回 false 。
- bool remove(int val) 如果存在，从集合中移除一个 val 项。如果该项存在，则返回 true ，否则返回 false 。注意，如果 val 在集合中出现多次，我们只删除其中一个。
- int getRandom() 从当前的多个元素集合中返回一个随机元素。每个元素被返回的概率与集合中包含的相同值的数量 线性相关 。

您必须实现类的函数，使每个函数的 平均 时间复杂度为 O(1) 。

注意：生成测试用例时，只有在 RandomizedCollection 中 至少有一项 时，才会调用 getRandom 。



##### 示例 1:
```
输入
["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
[[], [1], [1], [2], [], [1], []]
输出
[null, true, false, true, 2, true, 1]

解释
RandomizedCollection collection = new RandomizedCollection();// 初始化一个空的集合。
collection.insert(1);// 向集合中插入 1 。返回 true 表示集合不包含 1 。
collection.insert(1);// 向集合中插入另一个 1 。返回 false 表示集合包含 1 。集合现在包含 [1,1] 。
collection.insert(2);// 向集合中插入 2 ，返回 true 。集合现在包含 [1,1,2] 。
collection.getRandom();// getRandom 应当有 2/3 的概率返回 1 ，1/3 的概率返回 2 。
collection.remove(1);// 从集合中删除 1 ，返回 true 。集合现在包含 [1,2] 。
collection.getRandom();// getRandom 应有相同概率返回 1 和 2 。
```

##### 提示:
- -2<sup>31</sup> <= val <= 2<sup>31</sup> - 1
- insert, remove 和 getRandom 最多 总共 被调用 2 * 10<sup>5</sup> 次
- 当调用 getRandom 时，数据结构中 至少有一个 元素

##### 题解：
```rust
use rand::Rng;
use std::collections::{HashMap, HashSet};

struct RandomizedCollection {
    nums: Vec<i32>,
    indices: HashMap<i32, HashSet<usize>>,
}

impl RandomizedCollection {

    fn new() -> Self {
        Self {
            nums: vec![],
            indices: HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        self.nums.push(val);
        let entry = self.indices.entry(val).or_insert(HashSet::new());
        entry.insert(self.nums.len() - 1);

        entry.len() == 1
    }
    
    fn remove(&mut self, val: i32) -> bool {
       if let Some(s) = self.indices.get_mut(&val) {
           if let Some(&index) = s.iter().next() {
                s.remove(&index);
                let last_index = self.nums.len() - 1;
                let last_val = self.nums[last_index];
                self.nums[index] = last_val;
                let entry = self.indices.entry(last_val).or_insert(HashSet::new());
                entry.insert(index);
                entry.remove(&last_index);
                self.nums.pop();

                true
           } else {
               false
           }
       } else {
           false
       }
    }
    
    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0, self.nums.len());
        self.nums[index]
    }
}

```
