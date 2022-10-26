### [28. 找出字符串中第一个匹配项的下标](https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/)

给你两个字符串 haystack 和 needle ，请你在 haystack 字符串中找出 needle 字符串的第一个匹配项的下标（下标从 0 开始）。如果 needle 不是 haystack 的一部分，则返回  -1 。

##### 示例 1：
```
输入：haystack = "sadbutsad", needle = "sad"
输出：0
解释："sad" 在下标 0 和 6 处匹配。
第一个匹配项的下标是 0 ，所以返回 0 。
```

##### 示例 2：
```
输入：haystack = "leetcode", needle = "leeto"
输出：-1
解释："leeto" 没有在 "leetcode" 中出现，所以返回 -1 。
```

##### 提示：
- 1 <= haystack.length, needle.length <= 104
- haystack 和 needle 仅由小写英文字符组成

##### 题解：
```rust
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let L = haystack.len();
        let N = needle.len();
        let hb = haystack.as_bytes();
        let nb = needle.as_bytes();

        if N < 1 {
            return 0;
        }

        if L < N {
            return -1;
        }

        for i in (0..=L-N) {
            let mut pos = i;

            for j in (0..N) {
                if hb[pos] != nb[j] {
                    break;
                }
                pos += 1;
            }

            if pos - i == N {
                return i as i32
            }
        }

        -1
    }
}
```

```rust
use std::collections::HashMap;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let H = haystack.len();
        let N = needle.len();
        let hb = haystack.as_bytes();
        let nb = needle.as_bytes();

        if N == 0 {
            return 0;
        }

        if H < N {
            return -1;
        }

        let mut right = HashMap::new();

        for i in 0..N {
            right.insert(nb[i], i);
        }

        let mut skip = 0;
        let mut i = 0;

        while i <=  H - N {
            skip = 0;

            for j in (0..N).rev() {
                if hb[i + j] != nb[j] {
                    skip = match right.get(&hb[i + j]) {
                        Some(&v) => match j > v {
                            true => j - v,
                            false => 1
                        },
                        None => 1
                    };

                    break;
                }
            }

            if skip == 0 {
                return i as i32;
            }

            i += skip;
        }

        -1
    }
}
```

```rust
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let H = haystack.len();
        let N = needle.len();

        if N == 0 {
            return 0;
        }

        let mut next = vec![0];

        for i in 1..N {
            let mut j = next[i - 1];
            while j > 0 && needle[i] != needle[j] {
                j = next[j - 1];
            }

            next.push(if needle[i] == needle[j] { j + 1 } else { j });
        }

        let mut j = 0;

        for (i, &c) in haystack.iter().enumerate() {
            while j > 0 && c != needle[j] {
                j = next[j - 1];
            }

            if needle[j] == c {
                j += 1;
            }

            if j == N {
                return (i + 1 - j) as i32;
            }
        }

        -1
    }    
}
```

```rust
use std::collections::HashMap;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let H = haystack.len();
        let N = needle.len();

        if N == 0 {
            return 0;
        }

        if H < N {
            return -1;
        }

        let mut shift = HashMap::new();

        for i in (0..N).rev() {
            if !shift.contains_key(&needle[i]) {
                shift.insert(needle[i], N - i);
            }
        }

        let mut idx = 0;

        while idx <= H - N {
            let hay = &haystack[idx..idx + N];

            if hay == needle {
                return idx as i32;
            }

            if idx >= H - N {
                return -1;
            }

            idx = match shift.get(&haystack[idx + N]) {
                Some(v) => idx + v,
                None => idx + N + 1
            }
        }

        if idx + N > H {
            -1
        } else {
            idx as i32
        }
    }
}
```

`BF` `BM` `KMP` `Sunday`
