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
        let ss = StringShare::wrap(s);
        match self.strings.get(&ss) {
            Some(x) => x.clone(),
            None => {
                let x = StringShare::new(String::from(s));
                self.strings.insert(x.clone());

                x
            }
        }
    }

    pub fn del(&mut self, s: &str) -> bool {
        let ss = StringShare::wrap(s);
        self.strings.remove(&ss)
    }

    pub fn clear(&mut self) {
        self.strings.clear();
    }
}
