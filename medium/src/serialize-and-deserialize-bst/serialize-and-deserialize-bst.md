### [449. 序列化和反序列化二叉搜索树](https://leetcode.cn/problems/serialize-and-deserialize-bst/)
序列化是将数据结构或对象转换为一系列位的过程，以便它可以存储在文件或内存缓冲区中，或通过网络连接链路传输，以便稍后在同一个或另一个计算机环境中重建。

设计一个算法来序列化和反序列化 二叉搜索树 。 对序列化/反序列化算法的工作方式没有限制。 您只需确保二叉搜索树可以序列化为字符串，并且可以将该字符串反序列化为最初的二叉搜索树。

编码的字符串应尽可能紧凑。



##### 示例 1：
```
输入：root = [2,1,3]
输出：[2,1,3]
```

##### 示例 2：
```
输入：root = []
输出：[]
```

##### 提示：
- 树中节点数范围是 [0, 10<sup>4</sup>]
- 0 <= Node.val <= 10<sup>4</sup>
- 题目数据 保证 输入的树是一棵二叉搜索树。

##### 题解：
```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::str::SplitWhitespace;
struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        self.pre_order(&root)
    }

    fn pre_order(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            format!(
                "{} {} {}",
                node.borrow().val.to_string(),
                self.pre_order(&node.borrow().left),
                self.pre_order(&node.borrow().right),
            )
        } else {
            ",".to_string()
        }
    }
	
    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.len() == 0 {
            return None;
        }

        let mut vec = data.split_whitespace();

        self.constructor(&mut vec)
    }

    fn constructor(&self, iter: &mut SplitWhitespace) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(val) = iter.next() {
            if val != "," {
                let node = Rc::new(RefCell::new(TreeNode::new(val.parse::<i32>().unwrap())));
                node.borrow_mut().left = self.constructor(iter);
                node.borrow_mut().right = self.constructor(iter);
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }
}
```
