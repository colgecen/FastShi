# Gereksinimler

## 1. Genel Amaç

Türkçe Q klavye düzeninde, kullanıcının 10 parmak hızlı yazım becerisini geliştirmesini sağlayan,
masaüstünde çalışan (çevrimdışı da kullanılabilen) bir Rust uygulaması.

## 2. Temel Özellikler

### 2.1 Kelime Haznesi
- Uygulama, **yüzlerce/binlerce Türkçe kelime** içeren yerel bir veri kümesiyle gelir.
- Kelimeler zorluk seviyesine göre etiketlenir: `kolay` (kısa, sık kullanılan), `orta`, `zor`
  (uzun, nadir harf içeren — ör. Ğ, Ş, Ç, Ö, Ü, İ ağırlıklı).
- Kelime haznesi kategorilere ayrılabilir: genel kelimeler, sayılar, noktalama içeren cümleler,
  yalnızca alt satır (Z X C V B N M) / üst satır (Q W E R T Y U I O P) / orta satır (A S D F G H J K L)
  ağırlıklı kelimeler.
- Kullanıcı kendi kelime listesini `.txt`/`.json` olarak içe aktarabilir (opsiyonel, ileri aşama).
- **Kelimelerin nereden geldiği:** Kelimeler uygulamayla birlikte gelen yerel veri dosyalarında
  (`assets/word_lists/*.json`) saklanır — internete bağlı değildir, harici bir sunucudan çekilmez.
  Detay ve tam yol için `DATA_MODEL.md`'ye bakınız.
- **Rastgelelik / tekrarsızlık:** Her test başlatıldığında, o test için kullanılacak kelime havuzu
  (el moduna ve zorluk seviyesine göre filtrelenmiş) **karıştırılır** (shuffle). Aynı kelime,
  bir test içinde art arda (peş peşe) **tekrar etmemelidir**. Bir sonraki test tamamen farklı bir
  sırayla/karışımla başlar; havuz belirli bir eşiğin altına düşerse (ör. testin geri kalanı için
  yeterli benzersiz kelime kalmadıysa) havuz sıfırlanıp yeniden karıştırılarak devam edilir, yine
  de son gösterilen kelimenin hemen tekrarı engellenir. Bu sayede kullanıcıya "hep aynı kelimeler
  geliyor" hissi verilmez.

### 2.2 Yazım Testi (Standart Mod)
- Kullanıcı süre seçer (ör. 15 sn / 30 sn / 60 sn / 120 sn) veya kelime sayısı seçer (ör. 25 / 50 / 100 kelime).
- Ekranda kelime/kelime grupları gösterilir, kullanıcı yazdıkça anlık olarak doğru/yanlış karakterler
  renklendirilir (yeşil: doğru, kırmızı: yanlış).
- **Hata davranışı (kilitli/engelleyici mod):** Kullanıcı yanlış bir tuşa bastığında, o karakter
  **kırmızı** ile işaretlenir ve imleç **bir sonraki karaktere ilerlemez**. Kullanıcı doğru tuşa
  basana kadar kırmızı işaret ekranda kalır; doğru tuşa basıldığı an karakter yeşile döner ve
  imleç ilerler. (Bu, gerçek dünyadaki popüler yazım eğitim uygulamalarında kullanılan standart
  davranıştır — hatalı karakteri atlayıp devam etmek yerine, kullanıcı hatayı düzeltmeye zorlanır.)
  Bu davranış her hatayı `key_errors` tablosuna da kaydeder (bkz. `DATA_MODEL.md`).
- Test bitiminde şu metrikler gösterilir:
  - **WPM** (dakikada kelime) ve **CPM** (dakikada karakter)
  - **Doğruluk oranı** (%)
  - Toplam doğru/yanlış tuş vuruşu sayısı
  - En çok hata yapılan tuşlar/harfler (ısı haritası opsiyonel)

### 2.3 Tek El Modu (Öne Çıkan Özellik)
- Kullanıcı test başlamadan önce **"Sadece Sağ El"** veya **"Sadece Sol El"** modunu seçebilir.
- Seçilen moda göre, kelime haznesinden **yalnızca o elin kullandığı tuşlardan oluşan kelimeler**
  filtrelenip gösterilir (bkz. `HAND_MAPPING.md`).
- Klavye görselinde (varsa sanal klavye gösterimi), kullanılmayan elin tuşları soluk/pasif gösterilir.
- Bu mod hem süreli hem kelime-sayılı test ile birlikte çalışabilmeli.
- Eğer seçilen el için yeterli sayıda kelime yoksa (kelime haznesi kısıtlıysa), kullanıcıya
  "tek harf/hece" pratik modu sunulur (gerçek kelime yerine o elin tuşlarından rastgele oluşturulan
  heceler/diziler).

### 2.4 İstatistik ve İlerleme Takibi
- Her test sonucu yerel veritabanına (SQLite) kaydedilir.
- Kullanıcı geçmiş testlerini, WPM gelişim grafiğini ve el bazlı (sağ/sol/iki el) karşılaştırmalı
  istatistiklerini görebilir.
- Günlük/haftalık pratik özeti (streak, toplam süre) gösterilir.

### 2.5 Ayarlar
- Klavye düzeni: Türkçe Q (varsayılan); ileride Türkçe F desteği eklenebilir (mimaride esnek bırakılmalı).
- Ses efektleri (tuş sesi, hata sesi) açma/kapama.
- Tema (açık/koyu).
- Zorluk seviyesi seçimi.

## 3. Fonksiyonel Olmayan Gereksinimler

- **Performans:** Tuş vuruşu ile ekran güncellemesi arasında gecikme hissedilmemeli (<16ms hedef).
- **Çevrimdışı çalışma:** İnternet bağlantısı gerektirmemeli (kelime haznesi ve veritabanı yerel).
- **Platform desteği:** Windows, Linux, macOS (mümkünse tek kod tabanından).
- **Küçük dağıtım boyutu:** Tek binary + gömülü kaynaklar (kelime listesi vb.) tercih edilir.
- **Erişilebilirlik:** Klavye ile tam gezinme, yeterli kontrast.

## 4. Kapsam Dışı (v1 için)
- Çok oyunculu / online yarış modu
- Mobil uygulama
- Bulut senkronizasyonu
