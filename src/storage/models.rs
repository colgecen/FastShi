use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub id: Option<i64>,
    pub tarih: String,
    pub modu: String,
    pub sure_sn: u64,
    pub hedef_kelime_sayisi: Option<u32>,
    pub wpm: f64,
    pub cpm: f64,
    pub dogruluk_yuzde: f64,
    pub dogru_tus: u32,
    pub yanlis_tus: u32,
    pub zorluk: String,
}
