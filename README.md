# Hızlı Yazım Antrenörü (10 Parmak / Tek El Modlu) — Proje Prompt'u

Bu klasör, **Rust** ile geliştirilecek, Türkçe QWERTY (Q klavye) düzeninde 10 parmak hızlı
yazım eğitimi veren bir **masaüstü uygulaması** için hazırlanmış geliştirme dokümanlarını içerir.
Bu dosyaları doğrudan bir kodlama asistanına (Claude Code, Cursor, vb.) "bu spesifikasyona göre
uygulamayı kur" diyerek verebilirsin.

## Dosyalar

| Dosya | İçerik |
|---|---|
| `README.md` | Bu dosya — genel prompt ve proje özeti |
| `REQUIREMENTS.md` | Fonksiyonel/fonksiyonel olmayan gereksinimler, kullanıcı akışları |
| `ARCHITECTURE.md` | Teknoloji seçimleri, modül yapısı, klasör iskeleti |
| `DATA_MODEL.md` | Kelime haznesi formatı, istatistik ve sonuç veri modelleri |
| `HAND_MAPPING.md` | Türkçe Q klavyede sol/sağ el tuş haritası ve tek-el modu mantığı |
| `ROADMAP.md` | Geliştirme aşamaları (milestone'lar) |
| `WORKFLOW.md` | Her görev sonrası test/build/commit kuralları (Türkçe conventional commit) |

## Tek Cümlelik Özet (Prompt)

> Rust ile yazılmış, Türkçe Q klavye düzenine göre 10 parmak hızlı yazım öğreten bir masaüstü
> uygulaması geliştir. Uygulama, binlerce kelimelik yerel bir kelime haznesi barındırsın; süreli
> (zaman sınırlı) yazım testleri yapabilsin; kullanıcı isterse sadece **sağ el** veya sadece
> **sol el** ile pratik yapabileceği bir mod sunsun (klavye, seçilen ele göre sadece o elin
> kullandığı tuşlardan oluşan kelimeleri göstersin); WPM/CPM, doğruluk oranı, hatalı tuş
> istatistikleri gibi metrikleri hesaplayıp göstersin; sonuçları yerel olarak (SQLite veya JSON)
> saklasın.

## Nasıl Kullanılır

1. Bu klasördeki tüm `.md` dosyalarını bir kodlama asistanına (ör. Claude Code) ver.
2. "Bu dokümanlara göre Rust projesini `cargo new` ile oluştur ve `ROADMAP.md`'deki 1. aşamadan
   başla; her görev sonunda `WORKFLOW.md`'deki adımları uygula (test → build → Türkçe conventional
   commit)" diyerek geliştirmeye başlat.
3. Her görev bittikçe `WORKFLOW.md`'deki test/build/commit adımları uygulanır, sonra
   `ROADMAP.md`'deki bir sonraki maddeye geçilir.
