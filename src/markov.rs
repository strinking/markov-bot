extern crate rand;

use self::rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::rc::Rc;

const SEQUENCE_END: Rc<String> = Rc::new(String::from(""));

fn roulette_wheel<'a, T>(map: &'a HashMap<T, u32>, rng: &mut Rng) -> Option<&'a T> {
    let sum = map.values().sum() as f32;
    let mut rand = rng.next_f32();
    for (key, val) in map.iter() {
        let prob = val / sum;
        if rand < prob {
            return key;
        }

        rand -= prob;
    }
    panic!("No roulette selection made");
}

pub struct Markov {
    assocs: HashMap<Rc<String>, HashMap<Rc<String>, u32>>,
    start: HashMap<Rc<String>, u32>,
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
        let words = string.split(' ');
        let prev: Rc<String>;

        // Get first word
        match words.next() {
            Some(s) => {
                prev = self.get_string(s);
                let count = self.start.entry(prev).or_insert(0);
                *count += 1;
            },
            None => {
                self.associate(prev, SEQUENCE_END);
                return;
            }
        }

        let next: Rc<String>;

        // Get second word
        match words.next() {
            Some(s) => {
                next = self.get_string(s);
                self.associate(prev, next);
            },
            None => {
                self.associate(prev, SEQUENCE_END);
                return;
            }
        }

        for word in words {
            prev = next;
            next = word;
            self.associate(prev, next);
        }
        self.associate(next, SEQUENCE_END);
    }

    #[inline]
    fn get_string(&mut self, string: &str) -> Rc<String> {
        // TODO add pool
        Rc::new(String::from(string))
    }

    #[inline]
    fn associate(&mut self, prev: Rc<String>, next: Rc<String>) {
        let probs = self.map.entry(prev)
            .or_insert_with(HashMap::new);
        let count = probs.entry(next).or_insert(0);
        *count += 1;
    }

    pub fn generate(&mut self, length: u32) -> Option<String> {
        let mut rng = thread_rng();

        // Get starting word
        let word: &String;
        match roulette_wheel(&self.start, &rng) {
            Some(x) => {
                word = x;
            },
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
                    result.push_str(word);
                    word = roulette_wheel(&probs, &rng)
                        .expect("Probability map has no entries");
                },
                None => {
                    // Word has no associations, finish
                    break;
                },
            }
        }

        Some(result)
    }
}

