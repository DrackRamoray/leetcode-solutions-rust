### [148. 排序链表](https://leetcode.cn/problems/sort-list/)
给你链表的头结点 head ，请将其按 升序 排列并返回 排序后的链表 。



##### 示例 1：
![img.png](img.png)
```
输入：head = [4,2,1,3]
输出：[1,2,3,4]
```

##### 示例 2：
```
输入：head = [-1,5,3,4,0]
输出：[-1,0,3,4,5]
```

##### 示例 3：
```
输入：head = []
输出：[]
```

##### 提示：
- 链表中节点的数目在范围 [0, 5 * 10<sup>4</sup>] 内
- -10<sup>5</sup> <= Node.val <= 10<sup>5</sup>


##### 进阶：你可以在 O(n log n) 时间复杂度和常数级空间复杂度下，对链表进行排序吗？

##### 题解：
```rust
impl Solution {
    fn divide_lists(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut left = Some(Box::new(ListNode::new(0)));
        let mut right = Some(Box::new(ListNode::new(0)));
        let mut flag = true;
        let mut left_ptr = &mut left;
        let mut right_ptr = &mut right;
        while let Some(mut next_node) = head {
            head = next_node.next.take();
            if flag {
                left_ptr.as_mut().unwrap().next = Some(next_node);
                left_ptr = &mut left_ptr.as_mut().unwrap().next;
            } else {
                right_ptr.as_mut().unwrap().next = Some(next_node);
                right_ptr = &mut right_ptr.as_mut().unwrap().next;
            }
            flag = !flag;
        }
        (left.unwrap().next, right.unwrap().next)
    }
    fn merge_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(node1), None) => {
                Some(node1)
            },
            (None, Some(node2)) => {
                Some(node2)
            },
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_lists(Some(node1), node2.next);
                    Some(node2)
                }
            },
            _ => None
        }
    }
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let result = Self::divide_lists(head);
        match result {
            (Some(left_node), Some(right_node)) => {
                let left = Self::sort_list(Some(left_node));
                let right = Self::sort_list(Some(right_node));
                let head = Self::merge_lists(left, right);
                head
            },
            (Some(left_node), None) => {
                Some(left_node)
            },
            (None, Some(right_node)) => {
                Some(right_node)
            },
            _ => {
                None
            }
        }
    }
}
```
