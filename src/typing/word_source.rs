use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::hand_mode::key_map::{hand_of_char, Hand};

#[derive(Debug, Clone)]
pub struct WordEntry {
    pub kelime: String,
    pub zorluk: String,
}

pub struct WordSource {
    all_words: Vec<WordEntry>,
    pool: Vec<String>,
    last_word: Option<String>,
    current_filtered: Vec<String>,
}

impl WordSource {
    pub fn from_json(json_str: &str) -> Self {
        let data: serde_json::Value = serde_json::from_str(json_str).unwrap_or_default();
        let mut all_words = Vec::new();

        if let Some(kelimeler) = data["kelimeler"].as_array() {
            for item in kelimeler {
                if let (Some(kelime), Some(zorluk)) =
                    (item["kelime"].as_str(), item["zorluk"].as_str())
                {
                    all_words.push(WordEntry {
                        kelime: kelime.to_string(),
                        zorluk: zorluk.to_string(),
                    });
                }
            }
        }

        let pool: Vec<String> = all_words.iter().map(|w| w.kelime.clone()).collect();

        Self {
            all_words,
            pool,
            last_word: None,
            current_filtered: Vec::new(),
        }
    }

    pub fn filter(&mut self, hand: Option<Hand>, zorluk: &str) {
        self.current_filtered = self
            .all_words
            .iter()
            .filter(|w| {
                let zorluk_match = zorluk == "karisik" || w.zorluk == zorluk;
                let hand_match = match hand {
                    Some(h) => w.kelime.chars().all(|c| match hand_of_char(c) {
                        Some(ch) => ch == h,
                        None => true,
                    }),
                    None => true,
                };
                zorluk_match && hand_match
            })
            .map(|w| w.kelime.clone())
            .collect();

        if self.current_filtered.len() < 3 {
            self.current_filtered = self
                .all_words
                .iter()
                .filter(|w| match hand {
                    Some(h) => w.kelime.chars().all(|c| match hand_of_char(c) {
                        Some(ch) => ch == h,
                        None => true,
                    }),
                    None => true,
                })
                .map(|w| w.kelime.clone())
                .collect();
        }

        self.pool = self.current_filtered.clone();
        self.pool.shuffle(&mut thread_rng());
        self.last_word = None;
    }

    pub fn refill_and_shuffle(&mut self) {
        self.pool = self.current_filtered.clone();
        self.pool.shuffle(&mut thread_rng());
    }

    pub fn next_word(&mut self) -> String {
        if self.pool.is_empty() {
            self.refill_and_shuffle();
        }
        let mut word = self.pool.pop().unwrap_or_else(|| "hata".to_string());

        if Some(&word) == self.last_word.as_ref() && !self.pool.is_empty() {
            word = self.pool.pop().unwrap();
        }

        self.last_word = Some(word.clone());
        word
    }

    pub fn available_count(&self) -> usize {
        self.current_filtered.len()
    }
}
