#[derive(Debug, Clone)]
pub struct Metrics {
    pub wpm: f64,
    pub cpm: f64,
    pub dogruluk_yuzde: f64,
    pub dogru_tus: u32,
    pub yanlis_tus: u32,
}

impl Metrics {
    pub fn hesapla(dogru_tus: u32, yanlis_tus: u32, elapsed_secs: f64) -> Self {
        let toplam = dogru_tus + yanlis_tus;
        let dakika = elapsed_secs / 60.0;

        let wpm = if dakika > 0.0 {
            (dogru_tus as f64 / 5.0) / dakika
        } else {
            0.0
        };

        let cpm = if dakika > 0.0 {
            dogru_tus as f64 / dakika
        } else {
            0.0
        };

        let dogruluk_yuzde = if toplam > 0 {
            (dogru_tus as f64 / toplam as f64) * 100.0
        } else {
            0.0
        };

        Self {
            wpm,
            cpm,
            dogruluk_yuzde,
            dogru_tus,
            yanlis_tus,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_hesaplama() {
        let m = Metrics::hesapla(250, 10, 60.0);
        assert!((m.wpm - 50.0).abs() < 0.1);
    }

    #[test]
    fn test_dogruluk() {
        let m = Metrics::hesapla(90, 10, 60.0);
        assert!((m.dogruluk_yuzde - 90.0).abs() < 0.1);
    }

    #[test]
    fn test_sifir_tus() {
        let m = Metrics::hesapla(0, 0, 60.0);
        assert_eq!(m.wpm, 0.0);
        assert_eq!(m.dogruluk_yuzde, 0.0);
    }
}
