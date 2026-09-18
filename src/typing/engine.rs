#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMode {
    TwoHand,
    RightHand,
    LeftHand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Zorluk {
    Kolay,
    Orta,
    Zor,
}

impl Zorluk {
    pub fn as_str(&self) -> &'static str {
        match self {
            Zorluk::Kolay => "kolay",
            Zorluk::Orta => "orta",
            Zorluk::Zor => "zor",
        }
    }
}

pub struct TypingEngine {
    pub words: Vec<String>,
    pub word_index: usize,
    pub cursor_in_word: usize,
    pub has_error: bool,
    pub dogru_tus: u32,
    pub yanlis_tus: u32,
    pub words_completed: u32,
    pub total_words: Option<u32>,
    pub baslama: std::time::Instant,
    pub sure_saniye: u64,
    pub bitti: bool,
    pub hatali_tuslar: Vec<(char, char)>,
    pub typed_words: Vec<TypedWord>,
}

#[derive(Debug, Clone)]
pub struct TypedWord {
    pub word: String,
    pub typed_chars: Vec<Option<char>>,
    pub completed: bool,
}

impl TypingEngine {
    pub fn new(total_words: Option<u32>, sure_sn: u64) -> Self {
        Self {
            words: Vec::new(),
            word_index: 0,
            cursor_in_word: 0,
            has_error: false,
            dogru_tus: 0,
            yanlis_tus: 0,
            words_completed: 0,
            total_words,
            baslama: std::time::Instant::now(),
            sure_saniye: sure_sn,
            bitti: false,
            hatali_tuslar: Vec::new(),
            typed_words: Vec::new(),
        }
    }

    pub fn load_words(&mut self, new_words: Vec<String>) {
        self.words = new_words;
        self.word_index = 0;
        self.cursor_in_word = 0;
        self.has_error = false;
        self.typed_words.clear();
        for w in &self.words {
            self.typed_words.push(TypedWord {
                word: w.clone(),
                typed_chars: vec![None; w.chars().count()],
                completed: false,
            });
        }
    }

    pub fn current_word(&self) -> &str {
        if self.word_index < self.words.len() {
            &self.words[self.word_index]
        } else {
            ""
        }
    }

    pub fn on_key(&mut self, pressed: char) {
        if self.bitti {
            return;
        }
        if self.word_index >= self.words.len() {
            return;
        }

        let expected = self.current_word().chars().nth(self.cursor_in_word);
        match expected {
            Some(exp) if exp == pressed => {
                self.has_error = false;
                self.typed_words[self.word_index].typed_chars[self.cursor_in_word] = Some(pressed);
                self.cursor_in_word += 1;
                self.dogru_tus += 1;

                if self.cursor_in_word >= self.current_word().chars().count() {
                    self.typed_words[self.word_index].completed = true;
                    self.words_completed += 1;
                    if let Some(total) = self.total_words {
                        if self.words_completed >= total {
                            self.bitti = true;
                        }
                    }
                }
            }
            Some(exp) => {
                self.has_error = true;
                self.yanlis_tus += 1;
                self.hatali_tuslar.push((exp, pressed));
            }
            None => {}
        }
    }

    pub fn on_space(&mut self) {
        if self.bitti {
            return;
        }
        if self.word_index >= self.words.len() {
            return;
        }

        if self.cursor_in_word >= self.current_word().chars().count() {
            self.typed_words[self.word_index].completed = true;
            self.word_index += 1;
            self.cursor_in_word = 0;
            self.has_error = false;

            if self.word_index >= self.words.len() {
                self.bitti = true;
            }
        }
    }

    pub fn needs_more_words(&self) -> bool {
        self.word_index + 3 >= self.words.len()
    }

    pub fn is_finished(&self) -> bool {
        if self.bitti {
            return true;
        }
        if self.sure_saniye > 0 {
            return self.baslama.elapsed().as_secs() >= self.sure_saniye;
        }
        false
    }

    pub fn elapsed_secs(&self) -> f64 {
        self.baslama.elapsed().as_secs_f64()
    }
}
