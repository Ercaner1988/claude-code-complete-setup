# Claude Code Complete Setup (%100 Rust Engine)

Bu proje, **Claude Code** ortamınızı saniyeler içinde kurmanızı sağlanan **%100 Rust tabanlı** yüksek performanslı bir dağıtım ve yönetim sistemidir.

Eski Bash (`.sh`) ve Python (`.py`) scriptlerinin tamamı kaldırılarak tek bir yerel Rust ikili dosyasına (`claude-code-setup.exe`) dönüştürülmüştür.

## 🚀 Ana Versiyondan (Orijinal Repodan) Farklarımız

1. **%100 Saf Rust Mimarisi:** Kabuk (Shell) ve Python bağımlılıkları tamamen sıfırlanmıştır.
2. **Yerel Yol Normalizasyonu:** `/home/jb_remus` gibi sabit Linux yolları otomatik olarak kullanıcının yerel ortamına dönüştürülür.
3. **SQLite Tabanlı Hafıza Motoru:** Global hafıza notları gömülü SQLite veritabanına indekslenir ve anında arama yapılır.
4. **Dahili Otonom Ajan & Güvenlik:** Git branch koruması ve commit-hook yönetimi doğrudan Rust motoru üzerinden çalışır.

## 📦 Kurulum ve Kullanım
```bash
# Derleyin
cargo build --release

# Kurulum yapın
./target/release/claude-code-setup.exe install

# Tanı testlerini çalıştırın
./target/release/claude-code-setup.exe test
```

## 🛠️ Komutlar
- `install`: Tam kurulum ve yapılandırmayı yürütür.
- `update`: Yapılandırmaları günceller.
- `test`: Kurulum diagnostiklerini doğrular.
- `mcp-list`: Yapılandırılmış MCP sunucularını listeler.
- `memory-index`: Global hafızayı SQLite'a indeksler.
- `memory-search <kelime>`: Hafızada hızlı arama yapar.
- `security-audit`: Güvenlik ve Git durumunu denetler.
- `agent-workflow`: Otonom dal (branch) ve commit iş akışını çalıştırır.
