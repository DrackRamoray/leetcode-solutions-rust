pub struct Solution;

impl Solution {
    fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut trie = Box::new(Trie::new());

        for word in words {
            trie.insert(word);
        }

        let n = board.len();
        let m = board[0].len();
        let mut ans = vec![];

        for i in 0..n {
            for j in 0..m {
                Self::dfs( &mut board, &mut ans, &mut trie, i, j);
            }
        }

        ans.into_iter().collect()
    }

    fn dfs(board: &mut Vec<Vec<char>>, ans: &mut Vec<String>, trie: &mut Box<Trie>, i: usize, j: usize) {
        let n = board.len();
        let m = board[0].len();
        let c = board[i][j];
        if let Some(ref mut trie) = trie.children[c as usize - 'a' as usize] {
            board[i][j] = '{';

            if let Some(end) = trie.end.take() {
                ans.push(end);
            }

            if i > 0 {
                Self::dfs(board, ans, trie, i - 1, j);
            }

            if i + 1 < n {
                Self::dfs(board, ans, trie, i + 1, j);
            }

            if j > 0 {
                Self::dfs(board, ans, trie, i, j - 1);
            }

            if j + 1 < m {
                Self::dfs(board, ans, trie, i, j + 1);
            }

            board[i][j] = c;
        }
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>;27],
    end: Option<String>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut p = self;

        for &u in word.as_bytes() {
            p = p.children[u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        p.end = Some(word);
    }
}

#[cfg(test)]
mod tests {
    use assist::vec_stringify;
    use super::Solution;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['o', 'a', 'a', 'n'],
            vec!['e', 't', 'a', 'e'],
            vec!['i', 'h', 'k', 'r'],
            vec!['i', 'f', 'l', 'v']
        ];
        let words = vec_stringify!(vec!["oath", "pea", "eat", "rain"]);
        let mut ans = vec_stringify!(vec!["eat", "oath"]);
        let mut res = Solution::find_words(board, words);
        ans.sort();
        res.sort();
        assert_eq!(res, ans);
    }
}
