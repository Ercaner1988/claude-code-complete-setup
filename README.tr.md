# Claude Code Kurulum ve Dağıtım Motoru (Rust)

Bu proje, **Claude Code** ortamınızı saniyeler içinde kurmanızı sağlayan yüksek performanslı bir Rust kurulum motorudur. Tüm bağımlılıkları, MCP sunucularını ve güvenlik yapılandırmalarını yerel bir ikili dosya ile yönetir.

## 🚀 Temel Özellikler
- **Tam Rust Engine:** Bash script bağımlılığı olmadan, doğrudan ikili dosya üzerinden çalışma.
- **Güvenli Dağıtım:** Otomatik branch koruması ve pre-commit hookları.
- **Kapsamlı MCP Yönetimi:** 19 adet hazır MCP sunucusu ile otomatik kurulum.
- **Akıllı Hafıza:** Notlarınızı SQLite tabanlı hızlı arama sistemiyle yönetin.

## 📦 Kurulum
1. Derleyin: `cargo build --release`
2. Kurun: `./target/release/claude-code-setup.exe install`

## 🛠️ Komutlar
- `install`: Tam kurulumu başlatır.
- `update`: Yapılandırmaları günceller.
- `test`: Kurulum diagnostiklerini doğrular.
- `memory-search <kelime>`: Hafıza içerisinde arama yapar.

## 🛡️ Güvenlik
- **Branch Protection:** `main` veya `master` dalına doğrudan commit yapılmasını engeller.
- **Secret Scanner:** Kod içerisine hardcoded API anahtarı eklenmesini önler.
