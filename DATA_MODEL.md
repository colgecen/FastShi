# Veri Modeli

## 0. Kelimeler Nerede Saklanıyor?

- **Kaynak dosyalar:** Proje içinde `assets/word_lists/` klasöründe, her biri bir kategoriyi temsil
  eden `.json` dosyaları olarak tutulur (ör. `genel_tr.json`, `zor_tr.json`, `sayilar_tr.json`).
  Bu dosyalar **kaynak kod deposunun (git) bir parçasıdır**, yani projeyi klonlayan/derleyen
  herkeste otomatik olarak bulunur.
- **Dağıtım (binary'e gömme):** Derleme sırasında bu JSON dosyaları Rust'ın `include_str!("../../assets/word_lists/genel_tr.json")`
  makrosuyla **çalıştırılabilir dosyanın içine gömülür**. Böylece kurulum sonrası kullanıcı
  bilgisayarında ayrı bir dosya aramaya/bulmaya gerek kalmaz; uygulama tek bir `.exe`/binary olarak
  çalışır ve kelime haznesi zaten içindedir.
- **Kullanıcının kendi verisi (test sonuçları, ayarlar):** Bunlar ise kelime haznesinden farklı
  olarak **kullanıcının bilgisayarında, işletim sistemine göre standart config/veri klasöründe**
  saklanır (Rust'ta `directories` crate'i ile bulunur):
  - Linux: `~/.local/share/typing_trainer/` (veya `~/.config/typing_trainer/config.json`)
  - Windows: `%APPDATA%\typing_trainer\`
  - macOS: `~/Library/Application Support/typing_trainer/`
  - İçinde: `stats.sqlite` (test sonuçları, bkz. bölüm 2) ve `config.json` (ayarlar, bkz. bölüm 4).
- **İleride harici güncelleme isteniyorsa:** `include_str!` yerine çalışma zamanında kullanıcı
  veri klasöründeki bir `word_lists/` alt klasöründen okuma yapılabilir; bu, kelime haznesini
  uygulamayı yeniden derlemeden güncelleyebilmeyi sağlar (v1 kapsamında zorunlu değil, `ROADMAP.md`'ye
  ileri aşama olarak eklenebilir).

## 1. Kelime Haznesi Formatı (`assets/word_lists/*.json`)

```json
{
  "dil": "tr",
  "klavye_duzeni": "Q",
  "kategori": "genel",
  "kelimeler": [
    { "kelime": "kalem", "zorluk": "kolay" },
    { "kelime": "yazılım", "zorluk": "orta" },
    { "kelime": "değiştirilebilirlik", "zorluk": "zor" }
  ]
}
```

- `zorluk`: `kolay | orta | zor`
- Birden fazla dosya olabilir: `genel_tr.json`, `sayilar_tr.json`, `cumleler_tr.json` (noktalama içerir).
- Her kelime çalışma zamanında hangi tuşları içerdiği analiz edilerek (`hand_mode::key_map`)
  sol-el/sağ-el/karışık olarak sınıflandırılır — bu sınıflandırma dosyada önceden tutulmaz,
  dinamik hesaplanır (kelime listesi tek kaynaktan hem iki-el hem tek-el modunu besler).

## 2. Test Sonucu (SQLite tablosu: `test_results`)

| Alan | Tip | Açıklama |
|---|---|---|
| `id` | INTEGER PK | |
| `tarih` | TEXT (ISO8601) | Testin yapıldığı zaman |
| `mod` | TEXT | `iki_el` \| `sag_el` \| `sol_el` |
| `sure_sn` | INTEGER | Test süresi (saniye) |
| `hedef_kelime_sayisi` | INTEGER NULL | Kelime-sayılı modda hedef |
| `wpm` | REAL | Dakikada kelime |
| `cpm` | REAL | Dakikada karakter |
| `dogruluk_yuzde` | REAL | Doğruluk oranı |
| `dogru_tus` | INTEGER | |
| `yanlis_tus` | INTEGER | |
| `zorluk` | TEXT | `kolay \| orta \| zor \| karisik` |

## 3. Hata Detayı (opsiyonel tablo: `key_errors`)

| Alan | Tip | Açıklama |
|---|---|---|
| `id` | INTEGER PK | |
| `test_result_id` | INTEGER FK | |
| `beklenen_karakter` | TEXT | |
| `yazilan_karakter` | TEXT | |
| `sayac` | INTEGER | Bu testte kaç kez bu hata yapıldı |

Bu tablo, "en çok hata yapılan tuşlar" ısı haritası/özet için kullanılır.

## 4. Kullanıcı Ayarları (`~/.config/typing_trainer/config.json` benzeri)

```json
{
  "klavye_duzeni": "Q",
  "tema": "koyu",
  "ses_efektleri": true,
  "varsayilan_sure_sn": 60,
  "varsayilan_zorluk": "orta"
}
```
