struct Solution;

impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let n = words.len();
        let mut queue = Vec::new();
        let mut i = 0;
        let max_width = max_width as usize;

        while i < n {
            let mut count = 0;
            let mut tmp = Vec::new();

            while i < n && count < max_width {
                if count + words[i].len() > max_width {
                    break;
                }

                count += words[i].len() + 1;
                tmp.push(words[i].clone());
                i += 1;
            }

            if i == n {
                let last_line = tmp[..].join(&" ");
                let len = last_line.len();
                queue.push(last_line + &" ".repeat(max_width - len));
                break;
            }

            if tmp.len() == 1 {
                queue.push(tmp[..].join(&"") + &" ".repeat(max_width - count + 1));
            } else {
                let t = tmp.len() - 1;
                let paddings = max_width - (count - t - 1);
                let space = paddings / t;
                let mut extra = (paddings % t) as i32;
                let res = tmp.into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let times = space + if extra <= 0 { 0 } else { 1 };
                        extra -= 1;

                        if i == t {
                            x
                        } else {
                            x + &" ".repeat(times)
                        }
                    })
                    .collect::<String>();
                queue.push(res);
            }
        }

        queue
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let words = ["What","must","be","acknowledgment","shall","be"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        let max_width = 16;
        let ans = ["What   must   be".to_string(), "acknowledgment  ".to_string(), "shall be        ".to_string()];
        assert_eq!(Solution::full_justify(words, max_width), ans);
        let words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        let max_width = 20;
        let ans = ["Science  is  what we", "understand      well","enough to explain to","a  computer.  Art is","everything  else  we","do                  "].into_iter().map(|v| v.to_string()).collect::<Vec<_>>();
        assert_eq!(Solution::full_justify(words, max_width), ans);
    }
}
