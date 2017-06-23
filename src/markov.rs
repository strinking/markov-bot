extern crate rand;

use self::rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::rc::Rc;

const SEQUENCE_END: Rc<String> = Rc::new(String::from(""));

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
            let prev = next;
            let next = word;
            self.associate(prev, next);
        }
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


    pub fn generate(&mut self, length: u32) -> String {
        let mut rng = thread_rng();
        unimplemented!();
    }
}

     // TODO //
     pub fn generate(&mut self, length: u32) -> String {
         let mut random_number_generator = thread_rng();
         let keys = self.map.keys().collect::<Vec<&String>>();
         let key = random_number_generator
             .choose(&keys)
             .expect("no random value")
             .to_string();
         let mut sentence = key.clone();
 
         for _ in 0..length {
             let value = get_next_key(&self.map, &next_key(&sentence.to_string()));
             if value == "[STOP]" {
                 break;
             }
             sentence = format!("{} {}", sentence, value);
         }
         sentence
     }
 }
 
 fn get_next_key(map: &HashMap<String, HashMap<String, i32>>, key: &str) -> String {
     let mut choice: String = String::from("");
 
     match map.get(key) {
         Some(value) => {
             let mut sum_of_weights: i32 = 0;
             for idx in value.values() {
                 sum_of_weights += *idx;
             }
 
             let mut random: i32 = thread_rng().gen_range(0, sum_of_weights);
             let values = value.values().collect::<Vec<&i32>>();
             let keys = value.keys().collect::<Vec<&String>>();
 
             for i in 0..value.len() {
                 if random < *values[i] {
                     choice = format!("{}", *keys[i]);
                     break;
                 } else {
                     random -= *values[i];
                 }
             }
         }
 
         None => {
             choice = String::from("[STOP]");
         }
     }
     choice
 }
 
 fn next_key(key: &str) -> String {
     let last_word = key.split(' ').last().expect("couldn't get last word");
     String::from(last_word)
 }
