use super::share::StringShare;
use std::collections::HashSet;

pub struct StringPool<'a> {
    strings: HashSet<StringShare<'a>>,
}

impl<'a> StringPool<'a> {
    pub fn new() -> StringPool<'a> {
        StringPool { strings: HashSet::new() }
    }

    pub fn get(&mut self, s: &str) -> StringShare {
        match self.strings.get(s) {
            Some(x) => x.clone(),
            None => {
                let x = StringShare::new(s);
                self.strings.insert(x);

                x.clone()
            }
        }
    }

    pub fn del(&mut self, s: &str) -> bool {
        self.strings.remove(s)
    }

    pub fn clear(&mut self) {
        self.strings.clear();
    }
}
