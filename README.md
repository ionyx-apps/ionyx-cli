# 🚀 Ionyx Framework CLI

> **High-performance desktop apps with Rust, WebGPU, and Native Fusion.**

[![Ionyx Framework](https://img.shields.io/badge/Ionyx-Framework-blue?style=for-the-badge&logo=rust)](https://ionyx.app)
[![npm version](https://img.shields.io/npm/v/@ionyx-apps/cli.svg?style=for-the-badge)](https://www.npmjs.com/package/@ionyx-apps/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

Ionyx, modern masaüstü uygulamaları için geliştirilmiş, **Tauri** ve **Electron**'un gücünü **NW.js**'in kullanım kolaylığı (Native Fusion) ile birleştiren yeni nesil bir framework'tür.

---

## 🔥 Temel Özellikler

- 🦀 **Rust Core**: Güvenlik ve performans için TAO ve WRY üzerine kurulu.
- 🧬 **Native Fusion**: Backend state'ine frontend'den doğrudan (`window.fusion`) erişim. (Bi-directional Sync)
- 🎮 **WebGPU Support**: Yeni nesil grafik performansı tüm şablonlarda hazır.
- ⚕️ **Ionyx Doctor**: Geliştirme ortamınızı tek komutla denetleyin.
- 📦 **Native Bundling**: `.exe`, `.msi`, `.dmg`, `.appimage` ve taşınabilir ZIP paketleme.
- 🖼️ **Icon Support**: Uygulama ikonlarını tüm platformlar için otomatik dönüştürür.

---

## ⚡ Hızlı Başlangıç

### 1. Kurulum

Ionyx CLI'ı global olarak yükleyin:

```bash
npm install -g @ionyx-apps/cli
```

### 2. Ortam Kontrolü

Her şeyin hazır olduğundan emin olun:

```bash
ionyx doctor
```

### 3. Yeni Proje Oluşturun

İstediğiniz framework şablonuyla başlayın (React, Vue, Svelte, Leptos, Vanilla):

```bash
ionyx init my-app --template react
```

### 4. Geliştirmeye Başlayın

```bash
cd my-app
npm run ionyx:dev
```

---

## 🧬 Ionyx Native Fusion

NW.js'in "Unified Context" yapısından esinlenilen **Native Fusion**, frontend ve backend arasındaki sınırı kaldırır.

```javascript
// JS içinden Rust tarafındaki sistem verilerine anında erişin!
console.log(fusion.app_name); 
console.log(fusion.os);

// Değeri güncellediğinizde Rust tarafı otomatik senkronize olur
fusion.theme = "dark"; 
```

---

## 🏗️ Mimari Kıyaslama

| Özellik | Ionyx | Tauri | Electron |
| :--- | :---: | :---: | :---: |
| **Paket Boyutu** | ✅ ~3MB | ✅ ~5MB | ❌ ~100MB |
| **Native Fusion** | ✅ Var | ❌ Yok | ❌ Yok |
| **WebGPU** | ✅ Default | ⚠️ Limited | ✅ Var |
| **Performans** | 🚀 Native | 🚀 Native | 🐢 Node.js |
| **Güvenlik** | 🛡️ Sandbox | 🛡️ Sandbox | ⚠️ Açık |

---

## 📦 Dağıtım (Packaging)

Uygulamanızı paketlemek artık çok kolay:

```bash
# Windows için .exe ve .msi oluşturur
ionyx bundle

# Taşınabilir bir ZIP paketi oluşturur
ionyx bundle --portable
```

---

## 🤝 Katkıda Bulunma

Proje şu an aktif geliştirme aşamasındadır. PR'larınızı ve feedbacklerinizi bekliyoruz!

- **GitHub**: [ionyx-apps/ionyx](https://github.com/ionyx-apps/ionyx)
- **Website**: [ionyx.app](https://ionyx.app)

---

**🚀 Ionyx Framework - Desktop applications made fusion!**
