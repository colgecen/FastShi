# FastShi — Hızlı Yazım Antrenörü

Türkçe Q klavye düzeninde 10 parmak hızlı yazım becerisini geliştirmek için geliştirilmiş
masaüstü uygulaması. Tek el modu, WPM/CPM istatistikleri ve yerel kelime haznesi desteği sunar.

## Özellikler

- Turkish Q klavye düzeninde yazım pratiği
- Süreli veya kelime sayılı test modları
- Sol el / sağ el / iki el modu
- WPM, CPM ve doğruluk oranı istatistikleri
- Yerel kelime haznesi (çevrimdışı çalışır)
- SQLite ile kalıcı istatistik depolama
- Koyu/açık tema desteği

## Teknolojiler

- [Rust](https://www.rust-lang.org/)
- [egui / eframe](https://github.com/emilk/egui) — immediate-mode GUI
- [rusqlite](https://github.com/rusqlite/rusqlite) — SQLite bağımlılığı
- [serde / serde_json](https://serde.rs/) — JSON serileştirme

## Kurulum

```bash
# Depoyu klonlayın
git clone https://github.com/KULLANICI_ADI/FastShi.git
cd FastShi

# Derleyin ve çalıştırın
cargo run
```

## Proje Yapısı

```
FastShi/
├── assets/word_lists/    # Kelime haznesi dosyaları (JSON)
├── src/
│   ├── main.rs           # Uygulama giriş noktası
│   ├── app.rs            # Ana uygulama döngüsü
│   ├── typing/           # Yazım motoru ve metrikler
│   ├── hand_mode/        # Sol/sağ el filtresi
│   ├── storage/          # SQLite depolama
│   ├── ui/               # Arayüz ekranları
│   └── config.rs         # Kullanıcı ayarları
├── ARCHITECTURE.md       # Mimari kararlar
├── DATA_MODEL.md         # Veri modeli tanımları
├── HAND_MAPPING.md       # Tuş haritası
├── REQUIREMENTS.md       # Gereksinimler
└── ROADMAP.md            # Geliştirme yol haritası
```

## Geliştirme

Geliştirme süreci ve kurallar için `ROADMAP.md` dosyasına bakın.

```bash
# Testleri çalıştırın
cargo test

# Derlemeyi kontrol edin
cargo build

# Lint kontrolü
cargo clippy
```

## Lisans

Bu proje [MIT Lisansı](LICENSE) altında dağıtılır.
