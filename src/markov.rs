use std::collections::HashMap;
use rand::{thread_rng, Rng};
use std::hash::Hash;
use typemap::Key;
use regex::Regex;

fn get_next_word<'a, T: Eq + Hash>(map: &'a HashMap<T, u32>, rng: &mut Rng) -> Option<&'a T> {
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
    word_map: HashMap<String, HashMap<String, u32>>,
    starting_words: HashMap<String, u32>,
}

impl Key for Markov {
    type Value = Markov;
}

impl Markov {
    pub fn new() -> Markov {
        Markov {
            word_map: HashMap::new(),
            starting_words: HashMap::new(),
        }
    }

    pub fn parse(&mut self, string: &str) {
        let url_regex = Regex::new(r"[-a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*)").unwrap();
        let string = url_regex.replace_all(string, "");
        
        let mut words = string.split(' ');
        let mut previous_word: String;
        
        match words.next() {
            Some(word) => {
                previous_word = String::from(word);
                let count = self.starting_words.entry(previous_word.clone()).or_insert(0);
                *count += 1;
            }

            None => {
                println!("Empty message");
                return;
            }
        }

        let mut next_word: String;
        match words.next() {
            Some(word) => {
                next_word = String::from(word);
                self.associate(previous_word.clone(), next_word.clone());
            }

            None => {
                self.associate(previous_word.clone(), String::from(""));
                return;
            }
        }

        for word in words {
            previous_word = next_word.clone();
            next_word = String::from(word);
            self.associate(previous_word, next_word.clone());
        }
        self.associate(next_word.clone(), String::from(""));
    }

    #[inline]
    fn associate(&mut self, previous_word: String, next_word: String) {
        let probability = self.word_map.entry(previous_word).or_insert_with(HashMap::new);
        let count = probability.entry(next_word).or_insert(0);
        *count += 1;
    }

    pub fn generate(&self, length: u32, starting_word: Option<&String>) -> Option<String> {
        let mut rng = thread_rng();

        let mut word: &String;

        match starting_word {
            Some(start) => {
                word = start;
            }

            None => {
                match get_next_word(&self.starting_words, &mut rng) {
                    Some(start) => {
                        word = start;
                    }

                    None => {
                        return None;
                    }
                }
            }
        }

        let mut result = String::new();

        for _ in 0..length {
            match self.word_map.get(word) {
                Some(probability) => {
                    if !result.is_empty() {
                        result.push(' ');
                    }
                    result.push_str(&*word);

                    word =
                        get_next_word(&probability, &mut rng).expect("Probability map is empty");

                    if word.is_empty() {
                        if rng.gen::<bool>() {
                            result.push('.');
                        }
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
