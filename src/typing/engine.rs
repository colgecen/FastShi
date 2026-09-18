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

    pub fn from_str(s: &str) -> Self {
        match s {
            "kolay" => Zorluk::Kolay,
            "orta" => Zorluk::Orta,
            "zor" => Zorluk::Zor,
            _ => Zorluk::Orta,
        }
    }
}

pub struct TypingEngine {
    pub current_word: String,
    pub typed_so_far: String,
    pub cursor: usize,
    pub has_error: bool,
    pub dogru_tus: u32,
    pub yanlis_tus: u32,
    pub words_completed: u32,
    pub total_words: Option<u32>,
    pub baslama: std::time::Instant,
    pub sure_saniye: u64,
    pub bitti: bool,
    pub hatali_tuslar: Vec<(char, char)>,
}

impl TypingEngine {
    pub fn new(total_words: Option<u32>, sure_sn: u64) -> Self {
        Self {
            current_word: String::new(),
            typed_so_far: String::new(),
            cursor: 0,
            has_error: false,
            dogru_tus: 0,
            yanlis_tus: 0,
            words_completed: 0,
            total_words,
            baslama: std::time::Instant::now(),
            sure_saniye: sure_sn,
            bitti: false,
            hatali_tuslar: Vec::new(),
        }
    }

    pub fn set_word(&mut self, word: String) {
        self.current_word = word;
        self.typed_so_far.clear();
        self.cursor = 0;
        self.has_error = false;
    }

    pub fn on_key(&mut self, pressed: char) {
        if self.bitti {
            return;
        }

        let expected = self.current_word.chars().nth(self.cursor);
        match expected {
            Some(exp) if exp == pressed => {
                self.has_error = false;
                self.typed_so_far.push(pressed);
                self.cursor += 1;
                self.dogru_tus += 1;

                if self.cursor >= self.current_word.len() {
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

    pub fn total_tus(&self) -> u32 {
        self.dogru_tus + self.yanlis_tus
    }
}
