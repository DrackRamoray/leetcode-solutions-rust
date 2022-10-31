### [205. 同构字符串](https://leetcode.cn/problems/isomorphic-strings/)
给定两个字符串 s 和 t ，判断它们是否是同构的。

如果 s 中的字符可以按某种映射关系替换得到 t ，那么这两个字符串是同构的。

每个出现的字符都应当映射到另一个字符，同时不改变字符的顺序。不同字符不能映射到同一个字符上，相同字符只能映射到同一个字符上，字符可以映射到自己本身。



##### 示例 1:
```
输入：s = "egg", t = "add"
输出：true
```

##### 示例 2：
```
输入：s = "foo", t = "bar"
输出：false
```

示例 3：
```
输入：s = "paper", t = "title"
输出：true
```

##### 提示：
- 1 <= s.length <= 5 * 10<sup>4</sup>
- t.length == s.length
- s 和 t 由任意有效的 ASCII 字符组成

##### 题解：
```rust
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut v1 = vec![128;128];
        let mut v2 = vec![128;128];
        let ss = s.as_bytes();
        let tt = t.as_bytes();
        let L = ss.len();

        for n in 0..L {
            let i = (ss[n]) as usize;
            let j = (tt[n]) as usize;

            if (v1[i] != 128 && v1[i] != j) || (v2[j] != 128 && v2[j] != i) {
                return false;
            }

            v1[i] = j;
            v2[j] = i;
        }

        true
    }
}

```
