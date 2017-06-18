extern crate rand;

use self::rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct Markov {
    map: HashMap<String, HashMap<String, i32>>,
}

impl Markov {
    pub fn new() -> Markov {
        Markov { map: HashMap::new() }
    }

    pub fn parse(&mut self, string: &str) {
        let words = string.split(" ").collect::<Vec<&str>>();
        let word_count = words.len();

        for i in 0..word_count {
            if i + 1 < word_count {
                let key = String::from(words[i]);
                let second_key: String = format!("{}", words[i + 1]);
                let borrowed_second_key: String = format!("{}", words[i + 1]);


                if self.map.contains_key(&key) {
                    if self.map.get(&key).unwrap().contains_key(&second_key) {
                        self.insert(key, second_key);
                    } else {
                        self.insert(key, second_key);
                    }
                } else {
                    self.insert(key, borrowed_second_key);
                }
            }
        }
    }

    pub fn insert(&mut self, key: String, second_key: String) {
        if self.map.contains_key(&key) {
            let mut index = self.map.get_mut(&key).unwrap();
            if index.contains_key(&second_key) {
                let second_key_new = second_key;
                let entry = index.entry(second_key_new).or_insert(1);
                *entry += 1;
            } else {
                index.insert(second_key, 1);
            }
        } else {
            let mut new_index: HashMap<String, i32> = HashMap::new();
            new_index.insert(second_key, 1);
            self.map.insert(key, new_index);
        }
    }

    pub fn generate(&mut self, length: i32) -> String {
        let mut random_number_generator = thread_rng();
        let keys = self.map.keys().collect::<Vec<&String>>();
        let key = random_number_generator
            .choose(&keys)
            .expect("no random value")
            .to_string();
        let mut sentence = key.clone();
        let value = get_first_key(&self.map);
        sentence = format!("{} {}", sentence, value);

        for _ in 0..length {
            let value = get_next_key(&self.map, &next_key(&sentence.to_string()));
            sentence = format!("{} {}", sentence, value);
        }
        sentence
    }
}

fn get_first_key(map: &HashMap<String, HashMap<String, i32>>) -> String {
    let mut random_number_generator = thread_rng();
    let keys = map.keys().collect::<Vec<&String>>();
    let key = random_number_generator
        .choose(&keys)
        .expect("no random value")
        .to_string();

    match map.get(&key) {
        Some(_) => key,

        None => get_first_key(map),
    }
}

fn get_next_key(map: &HashMap<String, HashMap<String, i32>>, key: &String) -> String {
    let mut choice: String = String::from("");

    match map.get(*&key) {
        Some(value) => {
            let mut sum_of_weights: i32 = 0;
            for idx in value.values() {
                sum_of_weights += *idx;
            }

            let mut random: i32 = thread_rng().gen_range(0, sum_of_weights);
            let values = value.values().collect::<Vec<&i32>>();
            let keys = value.keys().collect::<Vec<&String>>();

            for i in 0..map.get(*&key).unwrap().len() {
                if random < *values[i] {
                    choice = format!("{}", *keys[i]);
                    break;
                } else {
                    random -= *values[i];
                }
            }
        }

        None => {
            choice = format!("{}", *key);
        }
    }
    choice
}

fn next_key(key: &str) -> String {
    let last_word = key.split(" ").last().expect("couldn't get last word");
    String::from(last_word)
}
