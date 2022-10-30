### [147. 对链表进行插入排序](https://leetcode.cn/problems/insertion-sort-list/)

给定单个链表的头 head ，使用 插入排序 对链表进行排序，并返回 排序后链表的头 。

插入排序 算法的步骤:

1. 插入排序是迭代的，每次只移动一个元素，直到所有元素可以形成一个有序的输出列表。

2. 每次迭代中，插入排序只从输入数据中移除一个待排序的元素，找到它在序列中适当的位置，并将其插入。

3. 重复直到所有输入数据插入完为止。

下面是插入排序算法的一个图形示例。部分排序的列表(黑色)最初只包含列表中的第一个元素。每次迭代时，从输入数据中删除一个元素(红色)，并就地插入已排序的列表中。

对链表进行插入排序。


![img.gif](img.gif)


##### 示例 1：
![img.png](img.png)
```
输入: head = [4,2,1,3]
输出: [1,2,3,4]
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入: head = [-1,5,3,4,0]
输出: [-1,0,3,4,5]
```

##### 提示：
- 列表中的节点数在 [1, 5000]范围内
- -5000 <= Node.val <= 5000

##### 题解：
```rust
impl Solution {
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;

        while let Some(mut h) = head {
            head = h.next.take();
            prev = Self::insert(prev, Some(h))
        }

        prev
    }

    fn insert(list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let val = list2.as_ref().unwrap().val;

        if let Some(mut n) = list1 {
            if n.val > val {
                list2.as_mut().unwrap().next = Some(n);
                list2
            } else {
                n.next = Self::insert(n.next.take(), list2);
                Some(n)
            }
        } else {
            list2
        }
    }
}
```
