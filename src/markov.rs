use std::collections::HashMap;
use diesel::pg::PgConnection;
use rand::{thread_rng, Rng};
use std::hash::Hash;
use typemap::Key;
use regex::Regex;
use schema::message::dsl::*;
use diesel::{FilterDsl, LoadDsl, SelectDsl, ExpressionMethods};

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(r"[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
}


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
    assoc_map: HashMap<String, HashMap<String, u32>>,
    starting_words: HashMap<String, u32>,
}

impl Key for Markov {
    type Value = Markov;
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            assoc_map: HashMap::new(),
            starting_words: HashMap::new(),
        }
    }

    pub fn parse(&mut self, string: &str) {
        let string = URL_REGEX.replace_all(string, "");
        let mut words = string.split(' ');
        let mut prev: String;

        match words.next() {
            Some(word) => {
                prev = String::from(word);
                let count = self.starting_words.entry(prev.clone()).or_insert(0);
                *count += 1;
            }

            None => return,
        }

        let mut next_word: String;
        match words.next() {
            Some(word) => {
                next_word = String::from(word);
                self.associate(prev.clone(), next_word.clone());
            }

            None => {
                self.associate(prev.clone(), String::from(""));
                return;
            }
        }

        for word in words {
            prev = next_word.clone();
            next_word = String::from(word);
            self.associate(prev, next_word.clone());
        }
        self.associate(next_word.clone(), String::from(""));
    }

    #[inline]
    fn associate(&mut self, prev: String, next_word: String) {
        let probability = self.assoc_map.entry(prev).or_insert_with(HashMap::new);
        let count = probability.entry(next_word).or_insert(0);
        *count += 1;
    }

    pub fn generate(&self, length: u32) -> Option<String> {
        let mut rng = thread_rng();
        let result = roulette_wheel(&self.starting_words, &mut rng)
            .map_or(None,
                    |x| self.generate_from_word(length, x));
        result
    }

    pub fn generate_from_word(&self, length: u32, starting_word: &str) -> Option<String> {
        let mut rng = thread_rng();
        let mut result = String::new();
        let mut word: &str = starting_word;

        for _ in 0..length {
            match self.assoc_map.get(word) {
                Some(probs) => {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(&*word);

                    word = roulette_wheel(&probs, &mut rng).expect("Probability map is empty");

                    if word.is_empty() {
                        break;
                    }
                }

                None => {
                    break;
                }
            }
        }
        Some(result)
    }
}


pub fn parse_messages(connection: &PgConnection, markov: &mut Markov) {
    let results = message.select(content).load::<String>(connection);

    for vector in results {
        for message_content in vector {
            markov.parse(message_content.as_str());
        }
    }
}

pub fn parse_user_messages(connection: &PgConnection, mut markov: &mut Markov, user_id: u64) {
    let results = message.select(content).filter(author_id.eq(user_id as i64)).load::<String>(connection);
                                                
    for vector in results {
        for msg in vector {
            markov.parse(msg.as_str());
        }
    }
}
