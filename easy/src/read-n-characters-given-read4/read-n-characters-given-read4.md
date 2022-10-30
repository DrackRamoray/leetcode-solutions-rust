### [157. 用 Read4 读取 N 个字符](https://leetcode.cn/problems/read-n-characters-given-read4/)

##### 题解：
```rust
impl Solution {
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;

        while ans < n {
            let mut tmp = vec![' ';4];
            let num = self.read4(&mut tmp);

            if num == 0 {
                return ans;
            }

            for j in 0..num {
                if (ans >= n) {
                    return ans;
                }

                buf[ans as usize] = tmp[j as usize];
                ans += 1;
            }
        }    

        ans
    }
}
```
