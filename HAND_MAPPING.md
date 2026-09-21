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

Her karakter için `hand_of_char` fonksiyonu ile hangi ele ait olduğu belirlenir. Bir kelimenin
belirli bir ele ait olup olmadığı, kelimedeki tüm harflerin aynı ele ait olup olmadığı kontrol
edilerek bulunur. Harf olmayan karakterler (boşluk, noktalama, rakamlar) filtreleme dışı bırakılır.

> Türkçe'de büyük/küçük İ-I ayrımına dikkat: `İ` (noktalı büyük İ) ve `i` sağ elde, `I` (noktasız
> büyük I) ve `ı` sol elde değerlendirilmelidir — bunlar Unicode'da farklı karakterlerdir, `to_lowercase()`
> Türkçe kurallarına göre çalışmayabilir. Bu yüzden Türkçe'ye özgü büyük/küçük harf dönüşümü
> elle (özel bir eşleme tablosuyla) yapılmalıdır.

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
