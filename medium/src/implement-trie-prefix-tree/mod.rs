#[derive(Default)]
pub struct Trie {
    children: [Option<Box<Trie>>;26],
    is_end: bool,
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

        p.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut p = self;

        for &u in word.as_bytes() {
            if let Some(ref child) = p.children[u as usize - 'a' as usize] {
                p = child;
            } else {
                return false;
            }
        }

        p.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut p = self;

        for &u in prefix.as_bytes() {
            if let Some(ref child) = p.children[u as usize - b'a' as usize]{
                p = child;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn it_works() {
        let mut trie = Trie::new();
        let word = "apple".to_string();
        trie.insert(word);
        let word = "apple".to_string();
        assert_eq!(trie.search(word), true);
        let word = "app".to_string();
        assert_eq!(trie.search(word), false);
        let word = "app".to_string();
        assert_eq!(trie.starts_with(word), true);
        let word = "app".to_string();
        trie.insert(word);
        let word = "app".to_string();
        assert_eq!(trie.search(word), true);
    }
}
