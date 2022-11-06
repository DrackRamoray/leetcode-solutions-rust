### [320. 列举单词的全部缩写](https://leetcode.cn/problems/generalized-abbreviation/)
单词的 广义缩写词 可以通过下述步骤构造：先取任意数量的 不重叠、不相邻 的子字符串，再用它们各自的长度进行替换。
- 例如，"abcde" 可以缩写为：
  - "a3e"（"bcd" 变为 "3" ）
  - "1bcd1"（"a" 和 "e" 都变为 "1"）
  - "5" ("abcde" 变为 "5")
  - "abcde" (没有子字符串被代替)
- 然而，这些缩写是 无效的 ：
  - "23"（"ab" 变为 "2" ，"cde" 变为 "3" ）是无效的，因为被选择的字符串是相邻的
  - "22de" ("ab" 变为 "2" ， "bc" 变为 "2")  是无效的，因为被选择的字符串是重叠的

给你一个字符串 word ，返回 一个由 word 的所有可能 广义缩写词 组成的列表 。按 任意顺序 返回答案。



##### 示例 1：
```
输入：word = "word"
输出：["4","3d","2r1","2rd","1o2","1o1d","1or1","1ord","w3","w2d","w1r1","w1rd","wo2","wo1d","wor1","word"]
```

##### 示例 2：
```
输入：word = "a"
输出：["1","a"]
```

##### 提示：
- 1 <= word.length <= 15
- word 仅由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn generate_abbreviations(word: String) -> Vec<String> {
        let words = word.chars().collect::<Vec<char>>();
        let mut ans = vec![];

        Self::dfs(&words, &mut ans, &mut String::new(), 0, 0);

        ans
    }

    fn dfs(words: &Vec<char>, ans: &mut Vec<String>, selected: &mut String, cur: usize, cnt: u8) {
        let n = selected.len();

        if cur == words.len() {
            if cnt > 0 {
                selected.push_str(&cnt.to_string());
            }

            ans.push(selected.clone());
        } else {
            Self::dfs(words, ans, selected, cur + 1, cnt + 1);

            if cnt > 0 {
                selected.push_str(&cnt.to_string());
            }

            selected.push(words[cur]);

            Self::dfs(words, ans, selected, cur + 1, 0);
        }

        selected.truncate(n);
    }
}
```
