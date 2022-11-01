### [234. 回文链表](https://leetcode.cn/problems/palindrome-linked-list/)
给你一个单链表的头节点 head ，请你判断该链表是否为回文链表。如果是，返回 true ；否则，返回 false 。



##### 示例 1：
![img.png](img.png)
```
输入：head = [1,2,2,1]
输出：true
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入：head = [1,2]
输出：false
```

##### 提示：
- 链表中节点数目在范围[1, 10<sup>5</sup>] 内
- 0 <= Node.val <= 9


##### 进阶：
- 你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？

##### 题解：
```rust
struct List {
    head: Option<Box<ListNode>>,
}

impl List {
    fn new(head: Option<Box<ListNode>>) -> Self {
        List { head }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    fn into_iter(self) -> IntoIter {
        IntoIter { list: self }
    }
}

struct IntoIter {
    list: List,
}

impl Iterator for IntoIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

impl Solution {
    fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let list = List::new(head);
        let vec: Vec<i32> = list.into_iter().collect();
        for (i, &v) in vec.iter().rev().enumerate() {
            if v != vec[i] {
                return false;
            }
        }
        true
    }
}
```
