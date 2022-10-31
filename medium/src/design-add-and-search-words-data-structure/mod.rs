#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>;26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: &str) {
        let mut p = self;

        for &u in word.as_bytes() {
            p = p.children[u as usize - 'a' as usize].get_or_insert_with(|| Box::new(Trie::new()));
        }

        p.is_end = true;
    }
}

struct WordDictionary {
    trie: Trie,
}

impl WordDictionary {

    fn new() -> Self {
        Self {
            trie: Trie::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        self.trie.insert(&word)
    }

    fn search(&self, word: String) -> bool {
        Self::dfs(word.as_bytes(), &self.trie, 0)
    }

    fn dfs(word: &[u8], node: &Trie, index: usize) -> bool {
        if index == word.len() {
            return node.is_end;
        }

        let u = word[index];

        if u != b'.' {
            if let Some(ref child) = node.children[u as usize - 'a' as usize] {
                if Self::dfs(word, child, index + 1) {
                    return true;
                }
            }
        } else {
            for child in node.children.iter() {
                if let Some(ch) = child {
                    if Self::dfs(word, ch, index + 1) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::WordDictionary;

    #[test]
    fn it_works() {
        let mut wd = WordDictionary::new();
        wd.add_word("bad".to_string());
        wd.add_word("dad".to_string());
        wd.add_word("mad".to_string());
        assert_eq!(wd.search("pad".to_string()), false);
        assert_eq!(wd.search("bad".to_string()), true);
        assert_eq!(wd.search(".ad".to_string()), true);
        assert_eq!(wd.search("b..".to_string()), true);
    }
}
