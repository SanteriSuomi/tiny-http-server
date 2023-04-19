use std::collections::HashMap;

struct Node<T> {
    children: HashMap<char, Node<T>>,
    is_end: bool,
    value: Option<T>,
}

impl<T> Node<T>
where
    T: Clone,
{
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_end: false,
            value: None,
        }
    }
}

pub struct Trie<T> {
    root: Node<T>,
}

impl<T> Trie<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self { root: Node::new() }
    }

    pub fn insert(&mut self, word: &str, value: T) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Node::new());
        }
        node.is_end = true;
        node.value = Some(value);
    }

    pub fn search(&self, word: &str) -> Option<T> {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(child) = node.children.get(&c) {
                node = child;
            } else {
                return None;
            }
        }
        if !node.is_end {
            return None;
        }
        Some(node.value.clone().unwrap())
    }
}
