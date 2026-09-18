# Sol El / Sağ El Tuş Haritası (Türkçe Q Klavye)

10 parmak standart öğretime göre klavye, dikey olarak iki yarıya bölünür. Aşağıdaki eşleme
`hand_mode::key_map` modülünde bir `HashMap<char, Hand>` veya `match` ifadesi olarak
uygulanmalıdır.

## 1. Varsayılan Eşleme

### Sol El
| Satır | Tuşlar |
|---|---|
| Sayı | `1 2 3 4 5` |
| Üst | `Q W E R T` |
| Orta (ev sırası) | `A S D F G` |
| Alt | `Z X C V B` |

### Sağ El
| Satır | Tuşlar |
|---|---|
| Sayı | `6 7 8 9 0` |
| Üst | `Y U I O P Ğ Ü` |
| Orta (ev sırası) | `H J K L Ş İ` |
| Alt | `N M Ö Ç` |

> Not: `T-Y` ve `G-H` ile `B-N` sınırı, geleneksel İngilizce QWERTY öğretiminden gelir ve bazı
> yazım öğretim kaynaklarında `T`/`B` sağ elin serçe parmağına, `Y`/`N` sol elin işaret parmağına
> verildiği alternatif eşlemeler de vardır. v1'de yukarıdaki **basit ve yaygın** eşleme kullanılsın;
> `key_map.rs` içinde bu tablo kolayca değiştirilebilecek şekilde (sabit bir `const` map olarak)
> tutulmalı ki ileride "eşlemeyi özelleştir" ayarı eklenebilsin.

## 2. Kelime Filtreleme Mantığı

```rust
enum Hand { Left, Right }

fn hand_of_char(c: char) -> Option<Hand> {
    match c.to_ascii_lowercase() {
        'q' | 'w' | 'e' | 'r' | 't' |
        'a' | 's' | 'd' | 'f' | 'g' |
        'z' | 'x' | 'c' | 'v' | 'b' => Some(Hand::Left),

        'y' | 'u' | 'i' | 'o' | 'p' | 'ğ' | 'ü' |
        'h' | 'j' | 'k' | 'l' | 'ş' | 'i' /* dotless İ ayrı ele dikkat */ |
        'n' | 'm' | 'ö' | 'ç' => Some(Hand::Right),

        _ => None, // boşluk, noktalama, rakamlarda ayrı ele göre değerlendirilebilir
    }
}

fn word_is_single_hand(word: &str, hand: &Hand) -> bool {
    word.chars().all(|c| {
        match hand_of_char(c) {
            Some(h) => std::mem::discriminant(&h) == std::mem::discriminant(hand),
            None => true, // harf olmayan karakterleri yok say
        }
    })
}
```

> Türkçe'de büyük/küçük İ-I ayrımına dikkat: `İ` (noktalı büyük İ) ve `i` sağ elde, `I` (noktasız
> büyük I) ve `ı` sol elde değerlendirilmelidir — bunlar Unicode'da farklı karakterlerdir, `to_lowercase()`
> Türkçe kurallarına göre çalışmayabilir (Rust'ın standart `to_ascii_lowercase` bunu doğru yapmaz).
> Bu yüzden Türkçe'ye özgü büyük/küçük harf dönüşümü elle (özel bir eşleme tablosuyla) yapılmalı.

## 3. Kelime Bulunamazsa Yedek Plan

Eğer seçilen el için (ör. `zor` zorluk + `sag_el` modu) kelime haznesinde yeterli kelime yoksa:

1. Önce zorluk filtresi gevşetilir (`zor` → `zor + orta`).
2. Yine yetersizse, gerçek kelime yerine **o elin tuşlarından rastgele hece/dizi üretimi**ne
   geçilir (ör. sağ el: `yuş`, `köl`, `mış` gibi anlamsız ama sadece o elin tuşlarını içeren
   diziler — ünlü/ünsüz dengesine dikkat edilerek üretilir, tamamen rastgele klavye gürültüsü
   olmamalı).
3. Kullanıcıya arayüzde küçük bir bilgi notu gösterilir: *"Bu el için yeterli gerçek kelime
   bulunamadı, hece pratiğine geçildi."*

## 4. Sanal Klavye Görseli (opsiyonel `ui/keyboard_widget.rs`)

- Aktif elin tuşları normal renkte, diğer elin tuşları soluk/gri gösterilir.
- Aktif kelimedeki bir sonraki karakter vurgulanır (ör. sarı çerçeve).
