### [341. 扁平化嵌套列表迭代器](https://leetcode.cn/problems/flatten-nested-list-iterator/)
给你一个嵌套的整数列表 nestedList 。每个元素要么是一个整数，要么是一个列表；该列表的元素也可能是整数或者是其他列表。请你实现一个迭代器将其扁平化，使之能够遍历这个列表中的所有整数。

实现扁平迭代器类 NestedIterator ：

- NestedIterator(List<NestedInteger> nestedList) 用嵌套列表 nestedList 初始化迭代器。
- int next() 返回嵌套列表的下一个整数。
- boolean hasNext() 如果仍然存在待迭代的整数，返回 true ；否则，返回 false 。

你的代码将会用下述伪代码检测：

```
initialize iterator with nestedList
res = []
while iterator.hasNext()
append iterator.next() to the end of res
return res
```

如果 res 与预期的扁平化列表匹配，那么你的代码将会被判为正确。



##### 示例 1：
```
输入：nestedList = [[1,1],2,[1,1]]
输出：[1,1,2,1,1]
解释：通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,1,2,1,1]。
```

##### 示例 2：
```
输入：nestedList = [1,[4,[6]]]
输出：[1,4,6]
解释：通过重复调用 next 直到 hasNext 返回 false，next 返回的元素的顺序应该是: [1,4,6]。
```

##### 提示：
- 1 <= nestedList.length <= 500
- 嵌套列表中的整数值在范围 [-10<sup>6</sup>, 10<sup>6</sup>] 内

##### 题解：
```rust
struct NestedIterator {
    index: usize,
    data: Vec<i32>,
}

impl NestedIterator {

    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut data = vec![];
        Self::flatten(&nestedList, &mut data);

        Self {
            index: 0,
            data,
        }
    }

    fn next(&mut self) -> i32 {
        let v = self.data[self.index];
        self.index += 1;
        v
    }
    
    fn has_next(&self) -> bool {
        self.index < self.data.len()
    }

    fn flatten(nestedList: &Vec<NestedInteger>, data: &mut Vec<i32>) {
        for item in nestedList.iter() {
            match item {
                NestedInteger::Int(v) => {
                    data.push(*v);
                },
                NestedInteger::List(inner_list) => {
                    Self::flatten(&inner_list, data);
                }
            }
        }
    }
}

```
