struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut solutions = Vec::new();
        let mut solution = Vec::new();
        let g = word_dict.into_iter().collect::<HashSet<_>>();

        Self::dfs(&s, &g, &mut solutions, &mut solution, 0);

        solutions
    }

    fn dfs(s: &str, g: &HashSet<String>, solutions: &mut Vec<String>, solution: &mut Vec<String>, pos: usize) {
        let n = s.len();

        if pos == n {
            solutions.push(solution.join(" "));
            return;
        }

        for i in (pos + 1)..=n {
            let w = s[pos..i].to_string();

            if g.contains(&w) {
                solution.push(w);
                Self::dfs(s, g, solutions, solution, i);
                solution.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::word_break("catsanddog".to_string(), vec_stringify!(["cat","cats","and","sand","dog"])), vec_stringify!(["cat sand dog","cats and dog"]));
        assert_eq!(Solution::word_break("pineapplepenapple".to_string(), vec_stringify!(["apple","pen","applepen","pine","pineapple"])), vec_stringify!(["pine apple pen apple", "pine applepen apple", "pineapple pen apple"]));
        assert_eq!(Solution::word_break("catsandog".to_string(), vec_stringify!(["cats","dog","sand","and","cat"])), vec![] as Vec<String>);
    }
}
