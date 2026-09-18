# Yol Haritası

## Aşama 0 — Proje İskeleti
- [x] `cargo new typing_trainer`
- [x] `egui`/`eframe`, `serde`, `serde_json`, `rusqlite` bağımlılıklarını ekle
- [x] Boş bir pencere açan minimal `main.rs`

## Aşama 1 — Temel Yazım Motoru
- [x] `assets/word_lists/genel_tr.json` içine ~200-300 örnek kelime ekle
- [x] `typing::engine`: kelime gösterme, klavye girişini yakalama, doğru/yanlış karakter takibi
- [x] Basit test ekranı (süre/kelime sayısı seçilebilir)
- [x] `typing::metrics`: WPM/CPM/doğruluk hesaplama
- [x] Sonuç ekranı

## Aşama 2 — Kalıcı Depolama
- [x] `storage::db`: SQLite şeması oluştur (`test_results` tablosu)
- [x] Test sonuçlarını kaydet
- [x] Basit geçmiş listesi ekranı

## Aşama 3 — Tek El Modu (öne çıkan özellik)
- [x] `hand_mode::key_map`: sol/sağ el eşlemesi + Türkçe büyük/küçük harf düzeltmesi
- [x] `word_source`: el moduna göre kelime filtreleme
- [x] Test başlatma ekranına "Sadece Sağ El / Sadece Sol El / İki El" seçeneği ekle
- [ ] Yetersiz kelime durumunda hece üretim yedek planı

## Aşama 4 — Kelime Haznesini Genişletme
- [x] Kelime sayısını 500+ çıkar
- [x] Zorluk seviyesi etiketleme (kolay/orta/zor)
- [ ] Kategoriler: sayılar, cümleler/noktalama

## Aşama 5 — İstatistik ve Görselleştirme
- [ ] `egui_plot` ile WPM gelişim grafiği
- [ ] El bazlı karşılaştırma (sağ el ort. WPM vs sol el vs iki el)
- [ ] En çok hata yapılan tuşlar özeti (`key_errors` tablosu)

## Aşama 6 — Cila (Polish)
- [ ] Sanal klavye görseli (aktif/pasif el vurgusu)
- [x] Tema desteği (siyah-beyaz)
- [x] Font desteği (Orbitron başlıklar, Space Mono metin)
- [x] Küçük harf yazım zorunluluğu
- [ ] Ses efektleri (opsiyonel)
- [ ] Ayarlar ekranı, config dosyası

## Aşama 7 — Dağıtım
- [ ] Windows/Linux/macOS için release build
- [x] Kelime haznesini binary'e gömme (`include_str!`)
