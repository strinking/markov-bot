use std::collections::HashSet;
use std::sync::Arc;

pub struct StringPool {
    strings: HashSet<Arc<String>>,
}

impl<'a> StringPool {
    pub fn new() -> Self {
        StringPool { strings: HashSet::new() }
    }

    pub fn get(&mut self, s: &str) -> Arc<String> {
        match self.strings.get(s) {
            Some(arc) => arc.clone(),
            None => {
                let arc = Arc::new(String::from(s));
                self.strings.insert(arc.clone());

                arc
            }
        }
    }

    pub fn del(&mut self, key: &str) -> bool {
        self.strings.remove(key)
    }

    pub fn purge(&mut self) {
        self.strings.clear();
    }
}
