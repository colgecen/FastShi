# Geliştirme İş Akışı (Workflow)

Bu proje üzerinde çalışan kodlama asistanı, `ROADMAP.md` içindeki **her bir görev/checkbox**
tamamlandığında aşağıdaki adımları sırasıyla uygulamalıdır. Bir görev, bu adımlar tamamlanmadan
"bitti" sayılmaz.

## 1. Test

```bash
cargo test
```
- Tüm testler geçmeden bir sonraki adıma geçilmez.
- Eğer görev için henüz test yazılmadıysa, en azından ilgili modül için (ör. `typing::metrics`,
  `hand_mode::key_map`) temel bir birim testi eklenmelidir (`tests/` klasörü veya modül içi
  `#[cfg(test)]`).

## 2. Build

```bash
cargo build
```
- Uyarısız (mümkünse `cargo clippy` de temiz) bir build hedeflenir.
- Build hatası varsa commit atılmaz, önce hata düzeltilir.

## 3. Commit (Conventional Commits — Türkçe açıklama ile)

Format:

```
<tür>(<kapsam opsiyonel>): <kısa açıklama, Türkçe, emir kipi>

[opsiyonel gövde: neden/nasıl yapıldığı]
```

### Kullanılacak türler
| Tür | Ne zaman kullanılır |
|---|---|
| `feat` | Yeni bir özellik eklendiğinde (ör. tek el modu, süre seçimi) |
| `fix` | Hata düzeltmesi |
| `test` | Sadece test eklendiğinde/güncellendiğinde |
| `refactor` | Davranış değişmeden kod yeniden düzenlendiğinde |
| `docs` | Sadece `.md` dosyaları / yorum güncellemesi |
| `chore` | Bağımlılık, proje iskeleti, CI/CD gibi işler |
| `perf` | Performans iyileştirmesi |
| `style` | Biçimlendirme (davranış değişikliği yok) |

### Örnekler
```
feat(hand-mode): sağ el ve sol el kelime filtreleme eklendi
fix(metrics): WPM hesaplamasında sıfıra bölme hatası düzeltildi
test(hand-mode): Türkçe büyük/küçük İ-I dönüşümü için birim testleri eklendi
docs(roadmap): aşama 3 görevleri güncellendi
chore: rusqlite bağımlılığı eklendi
```

### Kurallar
- Commit mesajı **Türkçe** yazılır.
- Başlık satırı **50-72 karakteri** geçmemeye çalışılır, emir kipiyle yazılır ("eklendi" yerine
  "ekle" de kabul edilir — proje içinde tutarlı olsun yeter; varsayılan olarak geçmiş zaman/edilgen
  "eklendi", "düzeltildi" biçimi kullanılabilir).
- Her commit, **tek bir mantıksal değişikliği** kapsar (bir görev = genelde bir commit; görev
  büyükse birden fazla küçük commit'e bölünebilir, her biri kendi test+build adımından geçer).
- Commit atılmadan önce `cargo test` ve `cargo build` **mutlaka** başarıyla tamamlanmış olmalı.

## 4. Görev Sonu Kontrol Listesi

Her görev için:
- [ ] Kod yazıldı
- [ ] İlgili test(ler) eklendi/güncellendi
- [ ] `cargo test` → başarılı
- [ ] `cargo build` → başarılı (tercihen `cargo clippy` de temiz)
- [ ] Conventional commit (Türkçe açıklama) atıldı
- [ ] `ROADMAP.md` içindeki ilgili checkbox işaretlendi
