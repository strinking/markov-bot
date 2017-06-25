use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::hash::Hash;
use typemap::Key;

fn roulette_wheel<'a, T: Eq + Hash>(map: &'a HashMap<T, u32>, rng: &mut Rng) -> Option<&'a T> {
    let sum = map.values().sum::<u32>() as f32;
    let mut rand = rng.next_f32();
    for (key, val) in map.iter() {
        let prob = (*val as f32) / sum;
        if rand < prob {
            return Some(&key);
        }

        rand -= prob;
    }

    None
}

pub struct Markov {
    assocs: HashMap<String, HashMap<String, u32>>,
    start: HashMap<String, u32>,
}

impl Key for Markov {
    type Value = Markov;
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            assocs: HashMap::new(),
            start: HashMap::new(),
        }
    }

    pub fn parse(&mut self, string: &str) {
        // TODO: sanitize string of stuff like URLs
        let mut words = string.split(' ');

        // Get first word
        let mut prev: String;
        match words.next() {
            Some(s) => {
                prev = String::from(s);
                let count = self.start.entry(prev.clone()).or_insert(0);
                *count += 1;
            }
            None => {
                // Message is empty
                return;
            }
        }

        // Get second word
        let mut next: String;
        match words.next() {
            Some(s) => {
                next = String::from(s);
                self.associate(prev.clone(), next.clone());
            }
            None => {
                self.associate(prev.clone(), String::from(""));
                return;
            }
        }

        for word in words {
            // TODO strip punctuation and stuff
            prev = next;
            next = String::from(word);
            self.associate(prev.clone(), next.clone());
        }
        self.associate(next, String::from(""));
    }

    #[inline]
    fn associate(&mut self, prev: String, next: String) {
        let probs = self.assocs.entry(prev).or_insert_with(HashMap::new);
        let count = probs.entry(next).or_insert(0);
        *count += 1;
    }

    pub fn generate(&self, length: u32) -> Option<String> {
        let mut rng = thread_rng();

        // Get starting word
        let mut word: &String;
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
            match self.assocs.get(word) {
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
