use std::collections::{HashMap, BinaryHeap};
use std::u128::MAX;

mod string_u128;
use string_u128::StringU128;

#[derive(Debug, Default)]
pub struct WordCounter{
    map: HashMap<String, u128>,
}

impl WordCounter{

    pub fn get_count(&self, word: &str) -> u128 {
        match self.map.get(word) {
            Some(count) => *count,
            None => 0,
        }
    }

    fn increment_count(&mut self, word: String) {
        if self.get_count(&word) == MAX {
            panic!("Count of the word {} has overflowed", word);
        }
        *self.map.entry(word).or_default() += 1 as u128;
    }

    pub fn insert(&mut self, word: &str) {
        let mut s = String::default();
        for ch in word.chars() {
            if !ch.is_alphanumeric() {
                if !s.is_empty() {
                    if self.get_count(&s) == MAX {
                        panic!("Overflow");
                    }
                    self.increment_count(s);
                }
                s = String::default();
            }
            else{
                s.push(ch);
            }
        }
        if !s.is_empty() {
           self.increment_count(s); 
        }
    }

    pub fn print(&self) {
        let mut q: BinaryHeap<StringU128> = BinaryHeap::default();
        for (key, value) in &self.map {
            q.push(StringU128::new(key, value));
        }
        while !q.is_empty() {
            println!(
            "{} : {}",
                q.peek().as_deref().unwrap().get_string(),
                q.pop().unwrap().get_count()
            );
            
        }
    }
}


#[cfg(test)]
mod tests{
    use super::WordCounter;

    #[test]
    fn insert() {
        let mut counter = WordCounter::default();
        counter.insert("This is a test of the WordCounter, specifically the insert operation");
        assert_eq!(counter.get_count("the"), 2 as u128);
        assert_eq!(counter.get_count("this"), 0 as u128); 
        assert_eq!(counter.get_count("this"), 0 as u128);
        assert_eq!(counter.get_count("This"), 1 as u128);
    }
}
