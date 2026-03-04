# my-ionyx-app

An Ionyx Framework application with Rust + Leptos (WASM).

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v18 or higher)
- npm or yarn

### Installation

\`\`\`bash
# Install dependencies
npm install

# Build WASM
cargo build --target wasm32-unknown-unknown

# Start development server
npm run dev
\`\`\`

### Available Scripts

- \`npm run dev\` - Start the development server
- \`npm run build\` - Build for production
- \`npm run preview\` - Preview production build

## 📁 Project Structure

\`\`\`
my-ionyx-app/
├── src/              # Rust source code
├── target/           # Build output
├── pkg/              # WASM package
├── package.json      # Dependencies
├── Cargo.toml        # Rust configuration
├── ionyx.config.json  # Ionyx configuration
└── README.md         # This file
\`\`\`

## 🔧 Ionyx Features

This application includes:

- ✅ **File System Access** - Read/write files
- ✅ **Network Requests** - HTTP API calls
- ✅ **OS Information** - System details
- ✅ **Cross-platform** - Windows, macOS, Linux
- ✅ **Rust Backend** - High performance
- ✅ **Rust Frontend (WASM)** - Zero-overhead frontend

## 📚 Learn More

- [Ionyx Framework Documentation](https://github.com/ionyx-apps/ionyx)
- [Leptos Documentation](https://leptos.dev/)
- [Rust Documentation](https://doc.rust-lang.org/)

## 🤝 Contributing

1. Fork this repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

## 📄 License

This project is licensed under the MIT License.
