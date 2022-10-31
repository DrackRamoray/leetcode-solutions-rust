### [211. 添加与搜索单词 - 数据结构设计](https://leetcode.cn/problems/design-add-and-search-words-data-structure/)
请你设计一个数据结构，支持 添加新单词 和 查找字符串是否与任何先前添加的字符串匹配 。

实现词典类 WordDictionary ：

- WordDictionary() 初始化词典对象
- void addWord(word) 将 word 添加到数据结构中，之后可以对它进行匹配
- bool search(word) 如果数据结构中存在字符串与 word 匹配，则返回 true ；否则，返回  false 。word 中可能包含一些 '.' ，每个 . 都可以表示任何一个字母。


##### 示例：
```
输入：
["WordDictionary","addWord","addWord","addWord","search","search","search","search"]
[[],["bad"],["dad"],["mad"],["pad"],["bad"],[".ad"],["b.."]]
输出：
[null,null,null,null,false,true,true,true]

解释：
WordDictionary wordDictionary = new WordDictionary();
wordDictionary.addWord("bad");
wordDictionary.addWord("dad");
wordDictionary.addWord("mad");
wordDictionary.search("pad"); // 返回 False
wordDictionary.search("bad"); // 返回 True
wordDictionary.search(".ad"); // 返回 True
wordDictionary.search("b.."); // 返回 True
```

##### 提示：
- 1 <= word.length <= 25
- addWord 中的 word 由小写英文字母组成
- search 中的 word 由 '.' 或小写英文字母组成
- 最多调用 10<sup>4</sup> 次 addWord 和 search

##### 题解：
```rust
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>;26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: &str) {
        let mut p = self;

        for &u in word.as_bytes() {
            p = p.children[u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        p.is_end = true;
    }
}

struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {

    fn new() -> Self {
        Self {
            trie: Trie::new(),
        }
    }
    
    fn add_word(&mut self, word: String) {
        self.trie.insert(&word)
    }
    
    fn search(&self, word: String) -> bool {
        Self::dfs(word.as_bytes(), &self.trie, 0)
    }

    fn dfs(word: &[u8], node: &Trie, index: usize) -> bool {
        if index == word.len() {
            return node.is_end;
        }

        let u = word[index];

        if u != b'.' {
            if let Some(ref child) = node.children[u as usize - 'a' as usize] {
                if Self::dfs(word, child, index + 1) {
                    return true;
                }
            }
        } else {
            for child in node.children.iter() {
                if let Some(ch) = child {
                    if Self::dfs(word, ch, index + 1) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

```
