# گزارش بازیابی سورس steam-workshop-downloader

## جواب کوتاه

دیکامپایل واقعی (برگردوندن کد Rust خوانا) از این فایل **ممکن نیست** — برنامه با Rust
کامپایل شده به کد ماشین native و برخلاف .NET یا Python، چیزی از سورس اصلی داخل
فایل exe باقی نمونده. ولی خبر خوب: **اسکلت کامل پروژه رو از روی رشته‌ها و
متادیتای داخل باینری استخراج کردم** و یه پروژه‌ی قابل بیلدِ اولیه برات ساختم
که می‌تونی ادامه‌ش بدی.

## برنامه‌ت چیه (از روی باینری)

- **زبان/فریمورک:** Rust + Tauri **v2.12.1** (دسکتاپ‌اپ با بک‌اند Rust)
- **کرت اصلی:** `steam_workshop_downloader_lib`
- **محیط بیلد:** ویندوز، یوزر `AbolUp`، مسیر cargo registry استاندارد
- **حجم:** ~16MB، بدون فرانت‌اند جاسازی‌شده (پایین توضیح دادم)

## چی بازیابی شد (دقیق و قابل اتکا)

| مورد | جزئیات |
|---|---|
| درخت ماژول‌ها | ۱۰ فایل: `lib.rs`, `commands/mod.rs`, `queue/manager.rs`, `steam/api.rs`, `steam/resolver.rs`, `steamcmd/manager.rs`, `steamcmd/parser.rs`, `steamcmd/process.rs`, `storage/db.rs`, `organizer/file_organizer.rs` |
| API بک‌اند | **۱۶ کامند Tauri** با اسم آرگومان‌ها (لیست کامل در `src/commands/mod.rs`) |
| دیتابیس | **اسکیمای کامل SQLite** — ۵ جدول با همه‌ی ستون‌ها، کلمه‌به‌کلمه از باینری (`src/storage/db.rs`) |
| ایونت‌ها | ۵ ایونت `workshop://...` (queue-updated, queue-started, queue-stopped, item-completed, progress) |
| ارورها | ۶ واریانت enum ارور با پیام‌های دقیق (مثلاً SteamGuardRequired) |
| Steam API | endpointهای `GetPublishedFileDetails/v1` و `GetCollectionDetails/v1` + لینک steamcmd.zip |
| دیپندنسی‌ها | لیست کامل کرت‌ها **با نسخه‌ی دقیق** (مثلاً rusqlite 0.32.1, reqwest 0.12.28) — لیست کامل پایین گزارش |
| منطق صف | ریکاوری آیتم‌های نیمه‌کاره در استارت‌آپ، شمارش retry، سیستم progress |

## چی بازیابی نشد

1. **بدنه‌ی توابع** — منطق داخلی هر تابع فقط اسمبلیه؛ اسکلت با `todo!()` گذاشتم و از روی
   شواهد (SQL، رشته‌ها، لاگ‌ها) نوشتم هر تابع چیکار می‌کرده.
2. **فرانت‌اند (HTML/JS)** — هیچ اثری از UI برنامه داخل exe نیست. اگه UI جداگونه
   داشتی (پوشه‌ی dist)، اون قسمت رو باید از نو بنویسی. ولی چون کل API بک‌اند
   (۱۶ کامند + ۵ ایونت) رو داری، دقیقاً می‌دونی UI جدید باید چی صدا بزنه.

## قدم‌های بعدی پیشنهادی

1. `cargo new` با همین `Cargo.toml` — دیپندنسی‌ها دقیقن همونایی هستن که باهاشون بیلد گرفتی.
2. `src/storage/db.rs` رو همون‌طور که هست استفاده کن — اسکیما کلمه‌به‌کلمه‌ست و با
   دیتابیس‌های قدیمی کاربرانت سازگاره (`workshop_downloader.sqlite3`).
3. کامندها رو یکی‌یکی با `todo!()` پر کن؛ برای هر کدوم نوشتم چیکار می‌کرده.
4. UI رو از نو بساز و به همین ۱۶ کامند وصلش کن.

## لیست کامل دیپندنسی‌های استخراج‌شده (کرت + نسخه)

aho-corasick 1.1.5, anyhow 1.0.104, atomic-waker 1.1.2, base64 0.22.1/0.23.1,
brotli-decompressor 6.0.1, bytes 1.12.1, cfb 0.14.0, chrono 0.4.45,
cookie 0.18.2, crossbeam-channel 0.5.17, ctor 1.0.13, dpi 0.1.2,
form_urlencoded 1.2.2, futures-channel/core/util 0.3.34, glob 0.3.4,
hashbrown 0.17.1, http 1.5.0, http-body-util 0.1.5, httparse 1.10.1,
hyper 1.11.1, hyper-rustls 0.27.10, hyper-util 0.1.21,
icu_collections/normalizer 2.3.0, idna 1.1.0, indexmap 2.14.2,
infer 0.22.0, ipnet 2.12.2, itoa 1.0.18, lazy_static 1.5.1,
matchers 0.2.0, memchr 2.8.3, mime 0.3.17, miniz_oxide 0.9.1, mio 1.2.3,
muda 0.20.0, once_cell 1.21.4, parking_lot 0.12.5, percent-encoding 2.3.2,
phf/phf_shared 0.13.1, regex 1.13.1, regex-automata 0.4.18,
regex-syntax 0.8.11, reqwest 0.12.28, ring 0.17.14, rusqlite 0.32.1,
rustls 0.23.45, rustls-pki-types 1.15.1, rustls-webpki 0.103.15,
semver 1.0.28, serde 1.0.229, serde_json 1.0.151,
serialize-to-javascript 0.1.2, sharded-slab 0.1.7, smallvec 1.16.2,
socket2 0.6.5, softbuffer 0.4.8, tao 0.37.1, tauri 2.12.1,
tauri-runtime-wry 2.12.1, tauri-utils 2.10.1, thread_local 1.1.10,
time 0.3.55, tokio 1.53.1, tokio-rustls 0.26.6, tower 0.5.3,
tracing-core 0.1.36, tracing-log 0.2.0, tracing-subscriber 0.3.23,
unicode-segmentation 1.13.3, untrusted 0.9.0, url 2.5.8,
urlpattern 0.6.0, uuid 1.26.1, want 0.3.1, webview2-com 0.39.1,
windows 0.62.2, wry 0.57.0, zip 2.4.2
