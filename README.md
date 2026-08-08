# Claude Code Complete Setup (Rust Engine)

**EN** | **TR** | **AR**

---

## 🚀 Overview (EN) / Genel Bakış (TR) / نظرة عامة (AR)

**EN:** A high-performance, single-binary Rust-based deployment system for Claude Code. It automates configuration, toolchain setup, MCP server management, and security auditing.

**TR:** Claude Code için yüksek performanslı, tek ikili dosyalı Rust tabanlı dağıtım sistemi. Yapılandırmayı, araç zinciri kurulumunu, MCP sunucu yönetimini ve güvenlik denetimini otomatikleştirir.

**AR:** نظام نشر عالي الأداء يعتمد على لغة Rust لـ Claude Code. يقوم بأتمتة الإعداد، وإدارة خوادم MCP، والتدقيق الأمني.

---

## 🛠️ CLI Usage / CLI Kullanımı / استخدام واجهة الأوامر

**EN:** Install the Rust engine with `cargo build --release` and use the generated binary from `./target/release/`.

**TR:** Rust motorunu `cargo build --release` ile kurun ve oluşturulan ikili dosyayı `./target/release/` altından kullanın.

**AR:** قم بتثبيت محرك Rust باستخدام `cargo build --release` واستخدم الملف التنفيذي الناتج من `./target/release/`.

### Commands (Commands / Komutlar / الأوامر)

| Command (EN) | Description (EN) | Açıklama (TR) | الوصف (AR) |
| :--- | :--- | :--- | :--- |
| `install` | Full setup & configuration | Tam kurulum ve yapılandırma | الإعداد والتكوين الكامل |
| `update` | Update existing setup | Mevcut kurulumu günceller | تحديث الإعداد الحالي |
| `test` | Run diagnostic suite | Tanı ve doğrulama testleri | اختبارات التشخيص والتحقق |
| `mcp-list` | List MCP servers | MCP sunucularını listeler | سرد خوادم MCP |
| `memory-index`| Index memory notes | Hafıza notlarını ındeksle | فهرسة ملاحظات الذاكرة |
| `memory-search`| Search memory | Hafızada arama yap | البحث في الذاكرة |
| `security-audit`| Run security audit | Güvenlik denetimi çalıştır | تشغيل تدقيق أمني |

---

## 📋 Included MCP Servers / Dahili MCP Sunucuları / خوادم MCP المضمنة

| Category | Servers |
| :--- | :--- |
| **Core** | `filesystem`, `github`, `git`, `time`, `docker` |
| **Cloud/DB** | `aws`, `postgres`, `memory` |
| **AI/Search** | `perplexity`, `exa`, `sequential-thinking` |
| **Automation** | `puppeteer`, `firecrawl` |
| **Productivity** | `google-drive`, `google-maps`, `notion`, `slack` |
| **Social/Art** | `reddit`, `everart` |

---

## 📦 Installation & Deployment / Kurulum ve Dağıtım / التثبيت والنشر

### 1. Requirements (EN) / Gereksinimler (TR) / المتطلبات (AR)
- **EN:** Rust/Cargo, Node.js, Python, Git.
- **TR:** Rust/Cargo, Node.js, Python, Git.
- **AR:** Rust/Cargo, Node.js, Python, Git.

### 2. Deployment (EN) / Dağıtım (TR) / النشر (AR)
**EN:** Use the provided scripts which automatically invoke the Rust binary: `./setup.sh`, `./update.sh`.

**TR:** Rust ikili dosyasını otomatik çağıran sağlanan scriptleri kullanın: `./setup.sh`, `./update.sh`.

**AR:** استخدم البرامج النصية المتوفرة التي تستدعي برنامج Rust الثنائي تلقائياً: `./setup.sh`, `./update.sh`.

---

## 📚 Documentation / Dokümantasyon / التوثيق
**EN:** Check `docs/` for detailed guides on Security, MCP setup, and IDE Integration.

**TR:** Güvenlik, MCP kurulumu ve IDE entegrasyonu hakkında ayrıntılı kılavuzlar için `docs/` klasörüne bakın.

**AR:** تحقق من `docs/` للحصول على أدلة مفصلة حول الأمن، إعداد MCP، وتكامل بيئة التطوير.
