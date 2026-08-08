# Kassam Görev Direktifi — Özellik 1 (Dinamik MCP) + Özellik 2 (Semantik + Graph Memory)

Yönlendiren: Claude (Ercan onayladı, 2026). Uygulayan: **Kassam**. İnceleyen: Claude.
Bu dosya kendine-yeter — başka bir yere bakmana gerek yok.

## ÖNCE ÇÖZ (kod yazmadan yanıtla)
1. **Özellik 3 durumu:** Bu ağaçta `src/security.rs::run_security_audit` hâlâ **yalnız-rapor** (oto-fix yok). "Hallettik" denen oto-fix hangi branch'te? Merge/rebuild edilmediyse bu binary'de yok. Teyit et; gerekiyorsa main'e getir.
2. **Embedding motoru:** `fastembed` (onnxruntime C++ runtime iner) mi, yoksa `candle` (saf-Rust, "100% Rust" kimliğine sadık) mı? Öneri: fastembed (en kısa yol); onnxruntime bağımlılığı kabul değilse candle.

## Kurallar (ponytail — inceleme bunları zorlayacak)
- Mevcut yapıyı YENİDEN KULLAN: `installer::get_home_dir`, `mcp::McpServerConfig`, `memory_engine` SQLite bağlantı deseni.
- Tek-implementasyonlu soyutlama YOK; graph DB YOK; ANN index YOK (bu ölçekte); ayrı vektör-store sunucusu YOK; yeni DB dosyası YOK.
- Önemsiz-olmayan her parça **1 koşulabilir test** bırakır (`cargo test`).

---

## Özellik 2 — Semantik + Graph Memory
Dosyalar: `src/memory_engine.rs`, `src/cli.rs`, `src/main.rs`, `Cargo.toml`.

1. Embedding = **yerel, çevrimdışı** (yukarıdaki karar). MiniLM-L6-v2. Uzak API/OpenAI YOK.
2. Depo = **mevcut** `~/.claude/memory_index.db`. `knowledge_notes`'a `embedding BLOB` kolonu ekle; yeni tablo `note_edges(src TEXT, dst TEXT, tur TEXT, agirlik REAL)`.
3. Semantik arama = **brute-force kosinüs** (lineer). Yorum ekle: `// ponytail: lineer kosinüs; not > ~5k olursa ANN ekle`.
4. Graph kenarları: (a) `[[wikilink]]` ayrıştır (deterministik); (b) kosinüs > eşik → semantik kenar. Komşu/en-kısa-yol = Rust BFS.
5. Keyword yolu: mevcut `LIKE`'ı **FTS5**'e yükselt (bundled SQLite destekler, 0 yeni bağımlılık).
6. CLI: `memory-index` embedding+kenar üretir; `memory-search <q> [--semantic|--keyword|--hybrid]`; yeni `memory-related <note>`.

Kabul kriteri: geçici knowledge dizini indeksle → `--semantic` sıralı sonuç; `memory-related` wikilink komşuları; FTS5 keyword çalışır; kosinüs birim testi geçer.

---

## Özellik 1 — Dinamik MCP Parametre Yönetimi
Dosyalar: `src/mcp.rs`, `src/cli.rs`, `src/main.rs`. İki-konum tespitini paylaşılan `resolve_config_path()`'e çıkar.

1. CLI: `mcp-set <server> [--command X] [--arg A]... [--env K=V]...`, `mcp-unset <server> [--env K] [--arg ...]`, `mcp-enable/--disable <server>`. `mcp-list` kalır.
2. **KRİTİK:** düzenlemeyi `serde_json::Value` üstünden yap (bkz. `normalize_mcp_config`), **tipli-struct round-trip DEĞİL** — yoksa modellenmeyen alanlar (ör. `disabled`) sessizce silinir.
3. Yazmadan önce `.bak` yedek + atomik yaz (temp+rename). Bozuk JSON'da çökme, hata bildir.
4. enable/disable = silmeden (`disabled: true` bayrağı).
5. Fırsatken düzelt: `normalize_mcp_config` koruması `/mnt/...`'i geçirip değiştirmiyor (ölü dal).

Kabul kriteri: elle eklenmiş bilinmeyen alan `mcp-set`/`mcp-unset` sonrası KORUNUR; `.bak` oluşur; bozuk JSON çökmez; Value-koruyan round-trip testi geçer.

---

## Teslim
`cargo build --release` temiz + `cargo test` geçer + PR/diff. Claude diff'i inceler: ponytail korkulukları, MCP Value-koruma, secret/yol işleme.
