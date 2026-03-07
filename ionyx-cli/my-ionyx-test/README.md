# my-ionyx-test

An Ionyx Framework application with React + TypeScript + Vite.

## 🚀 Getting Started

### Prerequisites

- Rust (latest stable)
- Node.js (v18 or higher)
- npm or yarn

### Installation

\`\`\`bash
# Install dependencies
npm install

# Start development server
npm run dev
\`\`\`

### Available Scripts

- \`npm run dev\` - Start the development server (frontend only)
- \`npm run build\` - Build for production (frontend only)
- \`npm run preview\` - Preview production build
- \`npm run ionyx:dev\` - Start Ionyx application (backend + frontend)
- \`npm run ionyx:build\` - Build Ionyx application for production
- \`npm run ionyx\` - Run Ionyx application (alias for ionyx:dev)

### Quick Start

\`\`\`bash
# Start Ionyx application (recommended)
npm run ionyx:dev

# Or start frontend only
npm run dev

# Build for production
npm run ionyx:build
\`\`\`

## 📁 Project Structure

\`\`\`
my-ionyx-test/
├── src/              # React application source
├── public/           # Static assets
├── dist/             # Build output
├── package.json      # Dependencies
├── vite.config.ts     # Vite configuration
├── tsconfig.json      # TypeScript configuration
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
- ✅ **React Frontend** - Modern UI

## 📚 Learn More

- [Ionyx Framework Documentation](https://github.com/ionyx-apps/ionyx)
- [React Documentation](https://react.dev/)
- [Vite Documentation](https://vitejs.dev/)

## 🤝 Contributing

1. Fork this repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a pull request

## 📄 License

This project is licensed under the MIT License.
