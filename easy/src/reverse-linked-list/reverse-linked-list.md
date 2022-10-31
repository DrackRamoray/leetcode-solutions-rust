### [206. 反转链表](https://leetcode.cn/problems/reverse-linked-list/)
给你单链表的头节点 head ，请你反转链表，并返回反转后的链表。


##### 示例 1：
![img_1.png](img_1.png)
```
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
```

##### 示例 2：
![img.png](img.png)
```
输入：head = [1,2]
输出：[2,1]
```

##### 示例 3：
```
输入：head = []
输出：[]
```

##### 提示：
- 链表中节点的数目范围是 [0, 5000]
- -5000 <= Node.val <= 5000


##### 进阶：
- 链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？

##### 题解：
```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut node) = curr.take() {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}
```
