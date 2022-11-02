pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    fn alien_order(words: Vec<String>) -> String {
        let letters = words.iter().fold(vec![false; 256], |mut acc, word| {
            word.bytes().for_each(|u| acc[u as usize] = true);
            acc
        });
        let k: usize = letters.iter().fold(0, |acc, &b| acc + (if b { 1 } else { 0 }));
        let mut edges: Vec<Vec<u8>> = vec![vec![]; 256];

        for w in words.windows(2) {
            if w[0] == w[1] {
                continue;
            }
            if w[0].starts_with(&w[1]) {
                return "".to_string();
            }
            if let Some((t, h)) = w[0]
                .bytes()
                .zip(w[1].bytes())
                .skip_while(|(t, h)| t == h)
                .take(1)
                .next()
            {
                edges[t as usize].push(h);
            }
        }

        let mut indegree: Vec<usize> = vec![0; 256];
        for i in 0..256 {
            for &h in &edges[i] {
                indegree[h as usize] += 1;
            }
        }

        let mut queue: VecDeque<u8> = VecDeque::new();
        for i in 0..256 {
            if letters[i] && indegree[i] == 0 {
                queue.push_back(i as u8);
            }
        }

        let mut ans = "".to_string();
        while let Some(t) = queue.pop_front() {
            ans.push(t as char);
            let n = edges[t as usize].len();
            for i in 0..n {
                let h = edges[t as usize][i];
                indegree[h as usize] -= 1;
                if indegree[h as usize] == 0 {
                    queue.push_back(h);
                }
            }
        }

        if k == ans.len() {
            ans
        } else {
            "".to_string()
        }
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::alien_order(vec_stringify!(vec!["wrt","wrf","er","ett","rftt"])), "wertf".to_string());
    assert_eq!(Solution::alien_order(vec_stringify!(vec!["z","x"])), "zx".to_string());
    assert_eq!(Solution::alien_order(vec_stringify!(vec!["z","x","z"])), "".to_string());
}
