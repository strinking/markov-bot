use self::StringShareValue::{Ref, Str};
use std::sync::Arc;

#[derive(Clone, Hash)]
enum StringShareValue<'a> {
    Ref(Arc<String>),
    Str(&'a str),
}

#[derive(Clone, Hash)]
pub struct StringShare<'a> {
    val: StringShareValue<'a>,
}

impl<'a> StringShare<'a> {
    pub fn new(s: String) -> StringShare<'a> {
        StringShare { val: Ref(Arc::new(s)) }
    }

    pub fn wrap(s: &'a str) -> StringShare<'a> {
        StringShare { val: Str(s) }
    }

    pub fn as_str(&self) -> &str {
        match self.val {
            Ref(arc) => &*arc,
            Str(s) => s,
        }
    }
}

impl<'a> PartialEq for StringShare<'a> {
    fn eq(&self, other: &Self) -> bool {
        let x = self.as_str();
        let y = other.as_str();

        x == y
    }
}

impl<'a> Eq for StringShare<'a> {}
