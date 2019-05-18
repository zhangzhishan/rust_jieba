use std::collections::hash_map::HashMap;
use std::hash::Hash;

struct DictUnit {
    let word: char;
    let weight: double;
    let tag: &str;
}; // struct DictUnit

struct Dag {
    let pInfo: DictUnit;
    let weight: double;
    // offsets, nexts.first
    let nexts: vec!((i32, DictUnit));
    let nextPos: i32;
}

struct TrieNode {
    let next: HashMap<char, TrieNode*>;
    let ptValue: DictUnit;
}

struct Trie {
    let root: TrieNode;
}



struct Trie<K, V> where K: Eq+Hash+Clone, V: Clone {

    value: Option<V>,

    children: HashMap<K, Trie<K, V>>,

}



impl<K, V> Trie<K,V> where K: Eq+Hash+Clone, V: Clone {

    fn new() -> Trie<K, V> {

        Trie {

            value: None,

            children: HashMap::new(),

        }

    }


    fn insert(&mut self, path: Vec<K>, v: V) {

        if path.is_empty() {

            match self.value {

                Some(_) => {

                    panic!("key exists")

                },

                None => {

                    self.value = Some(v);

                },

            }

            return;

        }

        self.children.entry(path[0].clone())
            .or_insert(Trie::new())
            .insert(path[1..].to_vec(), v)

    }



    fn fetch(&self, path: Vec<K>) -> Option<V> {

        match path.len() {

            0 => self.value.clone(),

            _ => self.children.get(&path[0])

                    .unwrap()

                    .fetch(path[1..].to_vec())

        }

    }

}

