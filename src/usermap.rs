use std::collections::HashMap;
use markov::Markov;
use typemap::Key;

pub struct UserMap;

impl Key for UserMap {
    type Value = HashMap<u64, Markov>;
}
