### [411. 最短独占单词缩写](https://leetcode.cn/problems/minimum-unique-word-abbreviation/)
通过将任意数量的 不相邻 子字符串替换为它们的长度，可以完成对字符串的 缩写 。 例如，像 "substitution" 这样的字符串可以缩写为（但不限于）：

- "s10n" ("s ubstitutio n")
- "sub4u4" ("sub stit u tion")
- "12" ("substitution")
- "su3i1u2on" ("su bst i t u ti on")
- "substitution" (不替换子字符串)

注意："s55n" ("s ubsti tutio n") 不是 "substitution" 的有效缩写形式，因为它试图替换两个相邻的子字符串。

缩写的 长度 是未被替换的字母数加上被替换的字符串数。例如，缩写 "s10n" 的长度为 3（2 个字母 + 1 个子字符串），而 "su3i1u2on" 的长度为 9（6 个字母 + 3 子字符串）。

给你一个目标字符串 target 和一个字符串数组 dictionary 作为字典，为 target 找出并返回一个 最短 长度的缩写字符串，同时这个缩写字符串 不是 字典 dictionary 中其他字符串的缩写形式。如果有多个有效答案，可以返回其中任意一个。



##### 示例 1：
```
输入：target = "apple", dictionary = ["blade"]
输出："a4"
解释："apple" 的最短缩写形式为 "5" ，但这也是 "blade" 的缩写形式之一。
下一组最短缩写是 "a4" 和 "4e" ，其中 "4e" 也是 "blade" 的缩写形式之一，而 "a4" 不是。
因此，返回 "a4" 。
```

##### 示例 2：
```
输入：target = "apple", dictionary = ["blade","plain","amber"]
输出："1p3"
解释："5" 同时是 "apple" 和字典中所有单词的缩写形式。
"a4" 同时是 "apple" 和 "amber" 的缩写形式。
"4e" 同时是 "apple" 和 "blade" 的缩写形式。
"1p3"、"2p2" 和 "3l1" 是 "apple" 的下一组最短缩写形式。
因为它们不是字典中其他单词的缩写形式，返回其中任意一个都是正确的。
```

##### 提示：
- target.length == m
- dictionary.length == n
- 1 <= m <= 21
- 0 <= n <= 1000
- 1 <= dictionary[i].length <= 100
- 如果 n > 0 ，那么 log2(n) + m <= 21
- target 和 dictionary[i] 仅包含小写字符

##### 题解：
```rust
impl Solution {
    fn min_abbreviation(target: String, dictionary: Vec<String>) -> String {
        let n = target.len();
        let target = target.chars().collect::<Vec<char>>();
        let dictionary = dictionary.into_iter().filter(|s| s.len() == n).map(|s| s.chars().collect()).collect::<Vec<Vec<char>>>();
        let mut mask: u32 = 0;
        let mut ans: u32 = !0;

        Self::dfs(&dictionary, &mut mask, &mut ans, &target, 0);
        Self::abbr(&target, ans)
    }

    fn dfs(dictionary: &[Vec<char>], mask: &mut u32, ans: &mut u32, target: &[char], begin: usize) {
        let n = target.len();

        if begin == n {
            for word in dictionary.iter() {
                if Self::check(target, word, *mask, n) {
                    return;
                }
            }

            if Self::get_size(*mask, n) < Self::get_size(*ans, n) {
                *ans = *mask;
            }
        } else {
            Self::dfs(dictionary, mask, ans, target, begin + 1);
            *mask |= 1 << begin;
            Self::dfs(dictionary, mask, ans, target, begin + 1);
            *mask &= !(1 << begin);
        }
    }

    fn check(target: &[char], word: &[char], mask: u32, n: usize) -> bool {
        (0..n).all(|i| mask & 1 << i == 0 || target[i] == word[i])
    }

    fn abbr(target: &[char], mask: u32) -> String {
        let n = target.len();
        let mut i = 0;
        let mut ans = String::new();

        while i < n {
            if mask & 1 << i == 0 {
                let mut cnt = 1;

                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    cnt += 1;
                    i += 1;
                }

                ans.push_str(&cnt.to_string());
            } else {
                ans.push(target[i]);
            }

            i += 1;
        }

        ans
    }

    fn get_size(mask: u32, n: usize) -> usize {
        let mut i = 0;
        let mut ans = 0;

        while i < n {
            if mask & 1 << i == 0 {
                while i + 1 < n && mask & 1 << (i + 1) == 0 {
                    i += 1;
                }
            }

            ans += 1;
            i += 1;
        }
        ans
    }
}
```
