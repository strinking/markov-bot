use pool::StringPool;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use typemap::Key;

fn roulette_wheel<T: Eq + Hash>(map: &HashMap<Arc<T>, u32>, rng: &mut Rng) -> Option<Arc<T>> {
    let sum = map.values().sum::<u32>() as f32;
    let mut rand = rng.next_f32();
    for (key, val) in map.iter() {
        let prob = (*val as f32) / sum;
        if rand < prob {
            return Some(*key);
        }

        rand -= prob;
    }

    None
}

pub struct Markov {
    assocs: HashMap<Arc<String>, HashMap<Arc<String>, u32>>,
    start: HashMap<Arc<String>, u32>,
    pool: StringPool,
}

impl Key for Markov {
    type Value = Markov;
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            assocs: HashMap::new(),
            start: HashMap::new(),
            pool: StringPool::new(),
        }
    }

    pub fn parse(&mut self, string: &str) {
        // TODO: sanitize string of stuff like URLs
        let mut words = string.split(' ');

        // Get first word
        let prev: Arc<String>;
        match words.next() {
            Some(s) => {
                prev = self.pool.get(s);
                let count = self.start.entry(prev.clone()).or_insert(0);
                *count += 1;
            }
            None => {
                // Message is empty
                return;
            }
        }

        // Get second word
        let next: Arc<String>;
        match words.next() {
            Some(s) => {
                next = self.pool.get(s);
                self.associate(prev.clone(), next.clone());
            }
            None => {
                self.associate(prev.clone(), self.pool.get(""));
                return;
            }
        }

        let mut current = (prev, next);
        for word in words {
            // TODO strip punctuation and stuff
            current = (next, self.pool.get(word));
            self.associate(current.0, current.1);
        }
        self.associate(next, self.pool.get(""));
    }

    #[inline]
    fn associate(&mut self, prev: Arc<String>, next: Arc<String>) {
        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let count = probs.entry(next).or_insert(0);
        *count += 1;
    }

    pub fn generate(&self, length: u32) -> Option<String> {
        let mut rng = thread_rng();

        // Get starting word
        let word: Arc<String>;
        match roulette_wheel(&self.start, &mut rng) {
            Some(x) => {
                word = x;
            }
            None => {
                // Markov chain is empty
                return None;
            }
        }

        let mut result = String::new();
        for _ in 0..length {
            match self.assocs.get(&word) {
                Some(probs) => {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(&*word);
                    word =
                        roulette_wheel(&probs, &mut rng).expect("Probability map has no entries");

                    if word.is_empty() {
                        // End of sequence
                        if rng.gen::<bool>() {
                            result.push('.');
                        }
                        break;
                    }
                }
                None => {
                    // Word has no associations, finish
                    break;
                }
            }
        }

        Some(result)
    }
}
