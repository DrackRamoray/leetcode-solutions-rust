### [293. 翻转游戏](https://leetcode.cn/problems/flip-game/)

##### 题解：
```rust
impl Solution {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let mut ans = vec![];
        let s = current_state.as_bytes();
        let n = s.len() - 1;

        for i in 0..n {
            if s[i] == b'+' && s[i+1] == b'+' {
                ans.push(current_state[0..i].to_string() + "--" + &current_state[i+2..]);
            }
        }

        ans
    }
}
```
