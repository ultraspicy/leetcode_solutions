use std::ops::Deref;

#[derive(Default)]
pub struct Trie {
    is_word: bool,
    children: [Box<Option<Trie>>; 26],
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;

        for ch in word.chars() {
            let idx = ch as usize - 'a' as usize;
            node = node.children[idx].get_or_insert_with(|| *Box::new(Trie::new()));
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for ch in word.chars() {
            let idx = ch as usize - 'a' as usize;
            match node.children[idx].deref() {
                None => return false,
                Some(trie) => node = trie,
            }
        }
        node.is_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for ch in prefix.chars() {
            let idx = ch as usize - 'a' as usize;
            match node.children[idx].deref() {
                None => return false,
                Some(trie) => node = trie,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple");
        assert!(trie.contains("apple"));
        assert!(!trie.contains("app"));
        assert!(trie.starts_with("app"));
        trie.insert("app");
        assert!(trie.contains("app"));
    }
}