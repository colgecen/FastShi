# Katkıda Bulunma Rehberi

FastShi projesine katkıda bulunduğunuz için teşekkürler.

## Geliştirme Ortamı

- Rust (stable) yükleyin: https://rustup.rs/
- Editör olarak VS Code + rust-analyzer önerilir

## Branş (Branch) Kullanımı

1. Proje reposunu fork edin
2. Yeni bir branş oluşturun: `git checkout -b ozellik/adi`
3. Değişikliklerinizi提交 edin
4. Pull request açın

## Commit Kuralları

Türkçe conventional commits formatı kullanılır:

```
<tür>(<kapsam>): <kısa açıklama>

[tür]: feat | fix | test | refactor | docs | chore | perf | style
```

### Örnekler

```
feat(hand-mode): tek el modu eklendi
fix(metrics): WPM hesaplamasında sıfıra bölme hatası düzeltildi
test(engine): kelime filtreleme için birim testleri eklendi
docs: README güncellendi
```

### Kurallar

- Commit mesajları Türkçe yazılır
- Başlık satırı 72 karakteri geçmemeli
- Her commit tek bir mantıksal değişikliği kapsamalı
- Değişikliklerden önce `cargo test` ve `cargo build` çalıştırılmalı

## Kodlama Standartları

- `cargo clippy` uyarı vermemeli
- `cargo fmt` ile biçimlendirme yapılmalı
- Yeni özellikler için birim testi eklenmeli

## Pull Request Süreci

1. PR açıklamasında ne değiştiği ve neden değiştiği açıkça belirtilmeli
2. Tüm CI kontrolleri (test, build, clippy) geçmeli
3. En az bir inceleyici (reviewer) onayı gerekmeli
