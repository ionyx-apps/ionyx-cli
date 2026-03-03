# 🚀 Ionyx Framework

> Universal desktop framework with Rust backend and agnostic frontend

[![Ionyx Framework](https://img.shields.io/badge/Ionyx-Framework-blue?style=flat-square&logo=)](https://img.shields.io/badge/Ionyx-Framework-blue?style=flat-square&logo=)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://img.shields.io/badge/License-MIT-green.svg)
[![npm version](https://img.shields.io/npm/v/create-ionyx-app.svg)](https://www.npmjs.com/package/create-ionyx-app)
[![Rust](https://img.shields.io/badge/rust-orange.svg)](https://www.rust-lang.org/)
[![Node.js](https://img.shields.io/badge/node.js-green.svg)](https://nodejs.org/)

Ionyx Framework, Rust backend ve frontend-agnostik yapı ile modern masaüstü uygulamaları geliştirmek için tasarlanmış, Tauri ve Electron seviyesinde bir framework'tür.

## 🚀 Hızlı Başlangıç

### Kurulum
```bash
# Global kurulum (önerilir)
npm install -g ionyx

# Yeni proje oluştur (Global kuruluysa)
ionyx create my-app

# Veya npx ile direkt kullanım
npx ionyx create my-app
```

### Proje Oluşturma
```bash
# React + TypeScript
npm create ionyx-app my-react-app --template react

# Svelte + TypeScript
npm create ionyx-app my-svelte-app --template svelte

# Vue + TypeScript
npm create ionyx-app my-vue-app --template vue

# Vanilla JavaScript
npm create ionyx-app my-vanilla-app --template vanilla

# Rust + Leptos (WASM)
npm create ionyx-app my-rust-app --template leptos
```

### Geliştirme
```bash
cd your-app
npm run ionyx-dev
```

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
