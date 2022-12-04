use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if start == end {
            return 0;
        }

        let gene = vec!["A", "C", "G", "T"];
        let mut step = 1;
        let mut cnt = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        visited.insert(start);

        for w in bank.iter() {
            cnt.insert(w);
        }

        while queue.len() > 0 {
            let len = queue.len();
            for _ in 0..len {
                let item = queue.pop_front().unwrap();

                for j in 0..8 {
                    for g in gene.iter() {
                        if g != &item.get(j..=j).unwrap() {
                            let s = item.get(0..j).unwrap_or("").to_string() + (*g) + item.get(j+1..).unwrap_or("");
                            if !visited.contains(&s) && cnt.contains(&s) {
                                if s.eq(&end) {
                                    return step;
                                }

                                queue.push_back(s.clone());
                                visited.insert(s);
                            }
                        }
                    }
                }
            }
            step += 1;
        }

        -1
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    assert_eq!(Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec_stringify!(["AACCGGTA"])), 1);
    assert_eq!(Solution::min_mutation("AACCGGTT".to_string(), "AAACGGTA".to_string(), vec_stringify!(["AACCGGTA","AACCGCTA","AAACGGTA"])), 2);
    assert_eq!(Solution::min_mutation("AAAAACCC".to_string(), "AACCCCCC".to_string(), vec_stringify!(["AAAACCCC","AAACCCCC","AACCCCCC"])), 3);
}
