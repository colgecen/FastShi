# Mimari

## 1. Teknoloji Seçimleri

| Katman | Öneri | Alternatif |
|---|---|---|
| Dil | Rust (stable) | — |
| GUI | `egui` / `eframe` (hızlı geliştirme, immediate-mode, tek binary'e kolayca gömülür) | `iced` (daha "native" görünüm), `Tauri` (web tabanlı arayüz + Rust backend) |
| Veritabanı | `rusqlite` (SQLite, gömülü, dosya tabanlı) | `sled` (saf Rust KV store) |
| Serileştirme | `serde` + `serde_json` (kelime haznesi ve config için) | — |
| Zamanlayıcı | `std::time::Instant` | — |
| Ses (opsiyonel) | `rodio` | — |

**Önerilen başlangıç:** `egui`/`eframe` + `rusqlite`. Sebep: tek binary, harici tarayıcı/web view
gerektirmez, klavye olaylarını düşük gecikmeyle yakalamak kolaydır, Türkçe karakter (UTF-8) render
konusunda sorun çıkarmaz.

## 2. Modül Yapısı (Cargo Workspace veya tek crate içinde modüller)

```
typing_trainer/
├── Cargo.toml
├── assets/
│   └── word_lists/           # bkz. DATA_MODEL.md
│       ├── genel_tr.json
│       ├── zor_tr.json
│       └── ...
├── src/
│   ├── main.rs                # uygulama giriş noktası, eframe kurulumu
│   ├── app.rs                 # ana App struct, egui update döngüsü
│   ├── typing/
│   │   ├── mod.rs
│   │   ├── engine.rs           # yazım oturumu state machine: aktif kelime, imleç, hata takibi,
│   │   │                       #   KİLİTLİ HATA MODU (yanlış tuşta kırmızı işaret + doğru tuşa
│   │   │                       #   basılana kadar imleç ilerlemez)
│   │   ├── metrics.rs          # WPM/CPM/doğruluk hesaplamaları
│   │   └── word_source.rs      # kelime haznesinden filtreleme + karıştırma (shuffle) + ardışık
│   │                           #   tekrarı engelleme (son gösterilen kelime bir daha hemen çıkmaz)
│   ├── hand_mode/
│   │   ├── mod.rs
│   │   └── key_map.rs          # HAND_MAPPING.md'deki tuş->el eşlemesi, filtreleme fonksiyonları
│   ├── storage/
│   │   ├── mod.rs
│   │   ├── db.rs                # SQLite bağlantısı, migration
│   │   └── models.rs            # TestResult, Session, Stats struct'ları
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── test_screen.rs       # aktif test ekranı
│   │   ├── results_screen.rs    # sonuç ekranı
│   │   ├── stats_screen.rs      # geçmiş/istatistik ekranı
│   │   ├── settings_screen.rs
│   │   └── keyboard_widget.rs   # opsiyonel sanal klavye görseli (aktif/pasif el vurgusu)
│   └── config.rs                # kullanıcı ayarları (JSON dosyası, ör. ~/.config/typing_trainer/)
└── tests/
    └── metrics_tests.rs
```

## 3. Veri Akışı (Özet)

1. Kullanıcı ayarları seçer (süre/kelime sayısı, el modu, zorluk) → `ui::test_screen`.
2. `typing::word_source`, `hand_mode::key_map` filtresini uygulayarak `assets/word_lists/*`
   içinden uygun kelimeleri çeker.
3. `typing::engine`, klavye olaylarını (egui input) dinler, her tuş vuruşunu doğru/yanlış olarak
   işaretler, imleci ilerletir.
4. Süre/kelime hedefi dolduğunda `typing::metrics` WPM/CPM/doğruluk hesaplar.
5. Sonuç `storage::db` içine yazılır, `ui::results_screen`'de gösterilir.
6. `ui::stats_screen`, geçmiş kayıtları sorgulayıp grafik (basit çizgi grafik `egui_plot` ile) çizer.

## 4. Örnek Mantık: Kilitli Hata Modu ve Kelime Seçimi

```rust
// typing/engine.rs — basitleştirilmiş state
struct TypingEngine {
    current_word: String,
    typed_so_far: String,      // kullanıcının şu ana kadar doğru yazdığı kısım
    cursor: usize,              // current_word içindeki hedef karakter indeksi
    has_error: bool,            // şu an kilitli bir hata var mı
    dogru_tus: u32,
    yanlis_tus: u32,
}

impl TypingEngine {
    fn on_key(&mut self, pressed: char) {
        let expected = self.current_word.chars().nth(self.cursor);
        match expected {
            Some(exp) if exp == pressed => {
                // doğru tuş: kırmızıyı temizle, imleci ilerlet
                self.has_error = false;
                self.cursor += 1;
                self.dogru_tus += 1;
            }
            Some(_) => {
                // yanlış tuş: kırmızı işaretle, imleç İLERLEMEZ
                self.has_error = true;
                self.yanlis_tus += 1;
                // -> UI bu karakteri kırmızı render eder
                // -> key_errors tablosuna (beklenen, yazılan) çifti kaydedilir
            }
            None => { /* kelime bitti, sıradaki kelimeye geç */ }
        }
    }
}
```

```rust
// typing/word_source.rs — karıştırma + ardışık tekrarsızlık
use rand::seq::SliceRandom;

struct WordSource {
    pool: Vec<String>,   // el moduna + zorluğa göre filtrelenmiş havuz
    last_word: Option<String>,
}

impl WordSource {
    fn refill_and_shuffle(&mut self, all_filtered: &[String]) {
        self.pool = all_filtered.to_vec();
        self.pool.shuffle(&mut rand::thread_rng());
    }

    fn next_word(&mut self) -> String {
        if self.pool.is_empty() {
            self.refill_and_shuffle(&self.original_filtered_list());
        }
        let mut word = self.pool.pop().unwrap();
        // ardışık tekrarı engelle: eğer son gösterilenle aynıysa bir sonrakini al / yeniden karıştır
        if Some(&word) == self.last_word.as_ref() && !self.pool.is_empty() {
            word = self.pool.pop().unwrap();
        }
        self.last_word = Some(word.clone());
        word
    }
}
```

## 5. Önemli Tasarım Kararları

- **Tek el modu filtresi**, kelime haznesi yüklenirken değil, test başlatılırken uygulanır — böylece
  aynı JSON kaynağı hem "iki el" hem "tek el" modları için kullanılabilir (bkz. `HAND_MAPPING.md`).
- Kelime listesi dosyaları derleme zamanında `include_str!` ile gömülebilir (dağıtımı kolaylaştırır)
  veya çalışma zamanında `assets/` klasöründen okunabilir (güncelleme kolaylığı). v1 için gömme
  önerilir, ileride harici güncellemeye açılabilir.
- Klavye düzeni (Q/F) bir `enum KeyboardLayout` ile soyutlanır ki ileride Türkçe F eklenebilsin.
