# 🚀 Ionyx Framework

> Universal desktop framework with Rust backend and agnostic frontend

[![Ionyx Framework](https://img.shields.io/badge/Ionyx-Framework-blue?style=flat-square&logo=)](https://img.shields.io/badge/Ionyx-Framework-blue?style=flat-square&logo=)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://img.shields.io/badge/License-MIT-green.svg)
[![npm version](https://img.shields.io/npm/v/ionyx.svg)](https://www.npmjs.com/package/ionyx)
[![Rust](https://img.shields.io/badge/rust-orange.svg)](https://www.rust-lang.org/)
[![Cargo](https://img.shields.io/badge/cargo-orange.svg)](https://doc.rust-lang.org/cargo/)

Ionyx Framework, Rust backend ve frontend-agnostik yapı ile modern masaüstü uygulamaları geliştirmek için tasarlanmış, Tauri ve Electron seviyesinde bir framework'tür.

## 🚀 Hızlı Başlangıç

### Gereksinimler

- **Rust**: [rustup.rs](https://rustup.rs) ile kurun
- **Node.js**: [nodejs.org](https://nodejs.org) ile kurun (v16+)
- **Cargo**: Rust ile birlikte gelir

### Kurulum

```bash
# NPM ile kurulum
npm install -g ionyx

# Veya Cargo ile (önerilir)
cargo install ionyx
```

### Proje Oluşturma

```bash
# Yeni Ionyx projesi oluştur
ionyx create my-app

# Veya template ile
ionyx create my-app --template react
```

### Geliştirme

```bash
cd my-app

# Development server başlat
ionyx dev

# Veya npm script ile
npm run dev
```

## 🏗️ Mimari

Ionyx, dört ana bileşenden oluşan minimalist bir mimari kullanır:

- **🦀 Rust Backend**: WRY WebView + TAO Windowing
- **⚡ CLI Tool**: Process orchestration ve environment management  
- **🌐 Frontend Agnostic**: React, Vue, Svelte, Vanilla JS
- **📦 Build System**: Cargo ile native compilation

### Özellikler

- ✅ **Sub-2MB** binary boyutu
- ✅ **<30ms** başlatma süresi  
- ✅ **Memory safe** Rust backend
- ✅ **Cross-platform**: Windows, macOS, Linux
- ✅ **Frontend agnostic**: İstediğiniz framework'ü kullanın

## 📚 Dokümantasyon

- [Website](https://ionyx.app)
- [Architecture Guide](https://ionyx.app/architecture)
- [API Reference](https://ionyx.app/docs)

## 🤝 Katkıda Bulunma

1. Fork edin
2. Feature branch oluşturun (`git checkout -b feature/amazing-feature`)
3. Commit edin (`git commit -m 'Add amazing feature'`)
4. Push edin (`git push origin feature/amazing-feature`)
5. Pull Request açın

## 📄 Lisans

Bu proje MIT lisansı altında lisanslanmıştır - detaylar için [LICENSE](LICENSE) dosyasına bakın.

## 🙏 Teşekkürler

- [Tauri](https://tauri.app) - İlham kaynağı
- [WRY](https://github.com/tauri-apps/wry) - WebView library
- [TAO](https://github.com/tauri-apps/tao) - Windowing library

---

**Ionyx** - Masaüstü uygulamaları için geleceğin framework'ü 🚀

### Build ve Dağıtım
```bash
# Production build
npm run ionyx-build

# Bundle (single binary)
npm run build:prod

# Installer oluştur
npm run build:installer
```

## ✨ Özellikler

### 🏗️ **Core Architecture**
- **Rust Backend**: TAO (windowing) + WRY (webview)
- **Frontend Agnostic**: React, Svelte, Leptos, Vanilla HTML/JS
- **IPC Bridge**: JavaScript-Rust iletişim
- **Custom Protocol**: `ionyx://` embedded assets
- **Cross-Platform**: Windows, macOS, Linux

### 📦 **Dağıtım**
- **Single Binary**: ~3MB tek dosya
- **Embedded Assets**: Frontend assets dahil
- **Installers**: ZIP, DMG, AppImage
- **Portable**: Her yere kopyalanabilir

### 🔒 **Güvenlik**
- **Permission System**: Güvenli dosya erişimi
- **Sandbox**: Kısıtlı çalışma alanı
- **Path Validation**: Güvenli path kontrolü
- **Config-Based**: Konfigürasyon tabanlı

### 🛠️ **CLI Araçları**
- **npmjs'de yayınlandı**: `npx create-ionyx-app`
- **Interactive Setup**: Frontend ve backend seçimi
- **Auto Dependencies**: Otomatik kurulum
- **Project Templates**: Hazır şablonlar

## 🎯 Neden Ionyx?

| Özellik | Ionyx Framework | Tauri | Electron |
|--------|----------------|------|--------|
| **Bundle Size** | ✅ ~3MB | ✅ ~5-10MB | ❌ ~50-100MB |
| **Performance** | ✅ Rust native | ✅ Rust native | ❌ Node.js |
| **CLI** | ✅ npmjs'de yayınlandı | ✅ npmjs'de yayınlandı | ❌ Manuel |
| **Security** | ✅ Sandbox + permissions | ✅ Sandbox + permissions | ❌ Daha az |
| **Development** | ✅ Hot reload + devtools | ✅ Hot reload + devtools | ✅ Hot reload + devtools |

## 📚 Belgeler

- [📖 Tam Özellik Listesi](./FEATURES.md)
- [🔧 API Belgelendirmesi](./docs/API.md)
- [📋 Örnek Projeler](./examples/)
- [🚀 Kurulum Kılavuzu](./docs/INSTALLATION.md)

## 🌐 Platform Desteği

- ✅ **Windows**: Windows 10/11 (x64)
- ✅ **macOS**: macOS 10.15+ (Intel/Apple Silicon)
- ✅ **Linux**: Ubuntu, Fedora, Arch (x64)

## 📦 NPM Paketi

```bash
npm install -g ionyx
npx ionyx create
```

## 🤝 Topluluk

- **Discord**: [Topluluk kanalı](https://discord.gg/ionyx)
- **GitHub**: [ionyx-apps/ionyx](https://github.com/ionyx-apps/ionyx)
- **Issues**: [Hata raporları](https://github.com/ionyx-apps/ionyx/issues)

## 📄 Lisans

Bu proje [MIT License](https://choosealicense.com/licenses/mit/) altında lisanslanmıştır. Detaylar için [LICENSE](./LICENSE) dosyasına bakın.

---

**🚀 Ionyx Framework - Desktop applications made simple!**
