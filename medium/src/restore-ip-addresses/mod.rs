struct Solution;

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&mut ans, &mut [0;4], s.as_bytes(), 0, 0);

        ans
    }

    fn dfs(ans: &mut Vec<String>, selected: &mut [u8;4], s: &[u8], cur: usize, start: usize) {
        if cur == 4 {
            if start == s.len() {
                ans.push(selected.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("."));
            }
            return;
        }

        if start == s.len() {
            return;
        }

        if s[start] == b'0' {
            selected[cur] = 0;
            Self::dfs(ans, selected, s, cur + 1, start + 1);
        }

        let mut addr = 0_i32;

        for i in start..s.len() {
            addr = addr * 10 + s[i] as i32 - b'0' as i32;

            if addr > 0 && addr < 256 {
                selected[cur] = addr as u8;
                Self::dfs(ans, selected, s, cur + 1, i + 1);
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::restore_ip_addresses("25525511135".into()), vec!["255.255.11.135".to_string(),"255.255.111.35".to_string()]);
        assert_eq!(Solution::restore_ip_addresses("0000".into()), vec!["0.0.0.0".to_string()]);
        assert_eq!(Solution::restore_ip_addresses("101023".into()), vec!["1.0.10.23".to_string(),"1.0.102.3".to_string(),"10.1.0.23".to_string(),"10.10.2.3".to_string(),"101.0.2.3".to_string()]);
    }
}
