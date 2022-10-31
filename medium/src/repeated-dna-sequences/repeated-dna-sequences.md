### [187. 重复的DNA序列](https://leetcode.cn/problems/repeated-dna-sequences/)
DNA序列 由一系列核苷酸组成，缩写为 'A', 'C', 'G' 和 'T'.。

- 例如，"ACGAATTCCG" 是一个 DNA序列 。

在研究 DNA 时，识别 DNA 中的重复序列非常有用。

给定一个表示 DNA序列 的字符串 s ，返回所有在 DNA 分子中出现不止一次的 长度为 10 的序列(子字符串)。你可以按 任意顺序 返回答案。



##### 示例 1：
```
输入：s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
输出：["AAAAACCCCC","CCCCCAAAAA"]
```

##### 示例 2：
```
输入：s = "AAAAAAAAAAAAA"
输出：["AAAAAAAAAA"]
```

##### 提示：
- 0 <= s.length <= 10<sup>5</sup>
- s[i]=='A'、'C'、'G' or 'T'

##### 题解：
```rust
impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let n = s.len();

        if n < 10 {
            return vec![];
        }

        let mut cache = std::collections::HashMap::new();
        let mut ans = vec![];

        for i in 0..=n-10 {
            *cache.entry(&s[i..i+10]).or_insert(0) += 1;

            if let Some(&cnt) = cache.get(&s[i..i+10]) {
                if cnt == 2 {
                    ans.push((s[i..i+10]).to_string());
                }
            }
        }

        ans
    }
}
```
