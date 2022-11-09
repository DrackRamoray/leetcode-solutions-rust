### [369. 给单链表加一](https://leetcode.cn/problems/plus-one-linked-list/)
给定一个用链表表示的非负整数， 然后将这个整数 再加上 1 。

这些数字的存储是这样的：最高位有效的数字位于链表的首位 head 。



##### 示例 1:
```
输入: head = [1,2,3]
输出: [1,2,4]
```

##### 示例 2:
```
输入: head = [0]
输出: [1]
```

##### 提示：
- 链表中的节点数在 [1, 100] 的范围内。
- 0 <= Node.val <= 9
- 由链表表示的数字不包含前导零，除了零本身。

##### 题解：
```rust
impl Solution {
    pub fn plus_one(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let carry = Self::plus(head.as_mut());

        if carry == 0 {
            head
        } else {
            let mut new_head = Box::new(ListNode::new(carry));
            new_head.as_mut().next = head;
            Some(new_head)
        }
    }

    fn plus(head: Option<&mut Box<ListNode>>) -> i32 {
        if let Some(mut h)= head {
            let val = h.val + Self::plus(h.next.as_mut());
            h.as_mut().val = val % 10;
            val / 10
        } else {
            1
        }
    }
}
```
