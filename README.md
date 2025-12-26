# QA Quiz App

Ứng dụng Quiz Desktop được xây dựng bằng Tauri v2 + SvelteKit + Sled Database với chủ đề quân đội Việt Nam.

![Version](https://img.shields.io/badge/version-0.1.0-green.svg)
![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)
![SvelteKit](https://img.shields.io/badge/SvelteKit-v2.9.0-orange.svg)

---

## 📋 Mục Lục

- [Giới Thiệu](#-giới-thiệu)
- [Công Nghệ Sử Dụng](#-công-nghệ-sử-dụng)
- [Yêu Cầu Hệ Thống](#-yêu-cầu-hệ-thống)
- [Cài Đặt & Chạy Dự Án](#-cài-đặt--chạy-dự-án)
- [Cấu Trúc Thư Mục](#-cấu-trúc-thư-mục)
- [Kiến Trúc Hệ Thống](#-kiến-trúc-hệ-thống)
- [Cấu Hình](#-cấu-hình)
- [Scripts & Commands](#-scripts--commands)
- [Database Structure](#-database-structure)
- [API Commands](#-api-commands)
- [Theme & Styling](#-theme--styling)
- [Build & Deploy](#-build--deploy)
- [Troubleshooting](#-troubleshooting)

---

## 🎯 Giới Thiệu

**QA Quiz App** là ứng dụng desktop cho phép người dùng:
- Đăng nhập với thông tin cá nhân (Họ tên, Cấp bậc, Đơn vị)
- Chọn chủ đề quiz và làm bài kiểm tra
- Xem kết quả và thống kê
- Quản lý câu hỏi (thêm, xóa)

**Đặc điểm:**
- 🖥️ Cross-platform Desktop App (Windows, macOS, Linux)
- ⚡ Single Page Application (SPA) với SvelteKit
- 🗄️ Embedded Database (Sled) - không cần cài đặt database riêng
- 🎨 Military-themed UI với Tailwind CSS v4
- 🔒 Type-safe với TypeScript

---

## 🛠️ Công Nghệ Sử Dụng

### Frontend Stack

| Công Nghệ | Phiên Bản | Mục Đích |
|-----------|----------|---------|
| **SvelteKit** | 2.9.0 | Framework frontend, routing, SPA |
| **Svelte** | 5.0.0 | Reactive UI components |
| **TypeScript** | 5.6.2 | Type safety |
| **Vite** | 6.0.3 | Build tool & dev server |
| **Tailwind CSS** | 4.1.18 | Utility-first CSS framework |
| **@tauri-apps/api** | 2.x | Tauri frontend API |
| **@sveltejs/adapter-static** | 3.0.6 | Static site generation |

### Backend Stack

| Công Nghệ | Phiên Bản | Mục Đích |
|-----------|----------|---------|
| **Rust** | Edition 2021 | Systems programming language |
| **Tauri** | 2.x | Desktop app framework |
| **Sled** | 0.34 | Embedded database |
| **Serde** | 1.x | Serialization/deserialization |
| **Serde JSON** | 1.x | JSON support |

### Development Tools

- **Node.js**: v16+ (khuyến nghị v18+)
- **Rust**: v1.70+ (khuyến nghị v1.75+)
- **pnpm/npm**: Package manager
- **Iconify**: Icon library (Solar icon set)
- **Google Fonts**: Typography (Work Sans, Montserrat)

---

## 💻 Yêu Cầu Hệ Thống

### macOS
```bash
# Yêu cầu:
- macOS 10.15+ (Catalina trở lên)
- Xcode Command Line Tools
- Node.js 16+
- Rust 1.70+

# Cài đặt prerequisites:
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install node  # hoặc từ nodejs.org
```

### Windows
```bash
# Yêu cầu:
- Windows 10/11
- Microsoft Visual Studio C++ Build Tools
- Node.js 16+
- Rust 1.70+

# Cài đặt:
# Download và cài đặt từ:
# - Node.js: https://nodejs.org/
# - Rust: https://www.rust-lang.org/tools/install
# - VS Build Tools: https://visualstudio.microsoft.com/downloads/
```

### Linux
```bash
# Yêu cầu:
- Ubuntu 20.04+ / Debian 11+ / Fedora 36+
- Node.js 16+
- Rust 1.70+
- Development dependencies

# Cài đặt (Ubuntu/Debian):
sudo apt update
sudo apt install libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🚀 Cài Đặt & Chạy Dự Án

### 1. Clone Repository

```bash
git clone <repository-url>
cd QA-quiz-app
```

### 2. Cài Đặt Dependencies

```bash
# Cài đặt Node.js dependencies
npm install

# Rust dependencies sẽ tự động được cài khi build
```

### 3. Chạy Development Mode

```bash
# Cách 1: Chạy tất cả (khuyến nghị)
npm run tauri dev

# Cách 2: Chạy riêng lẻ (2 terminals)
# Terminal 1: Frontend dev server
npm run dev

# Terminal 2: Tauri window
cd src-tauri
cargo tauri dev
```

Ứng dụng sẽ tự động mở window desktop với hot-reload enabled.

### 4. Build Production

```bash
# Build frontend
npm run build

# Build app cho platform hiện tại
npm run tauri build

# Output sẽ ở: src-tauri/target/release/bundle/
```

---

## 📁 Cấu Trúc Thư Mục

```
QA-quiz-app/
├── src/                              # Frontend Source Code
│   ├── routes/                       # SvelteKit Routes
│   │   ├── +page.svelte             # Main page - Route switcher
│   │   ├── +layout.svelte           # Global layout & CSS import
│   │   └── +layout.ts               # SSR config (disabled for SPA)
│   │
│   ├── lib/                         # Shared Libraries
│   │   ├── components/              # Svelte Components
│   │   │   ├── LoginScreen.svelte   # Màn hình đăng nhập
│   │   │   ├── Dashboard.svelte     # Dashboard chính
│   │   │   ├── QuizCategories.svelte # Chọn chủ đề quiz
│   │   │   ├── QuizGame.svelte      # Màn hình làm bài quiz
│   │   │   └── ResultScreen.svelte  # Hiển thị kết quả
│   │   │
│   │   └── store.ts                 # Global state management (Svelte stores)
│   │
│   ├── app.css                      # Global CSS (Tailwind v4)
│   └── app.html                     # HTML template
│
├── src-tauri/                        # Tauri Backend (Rust)
│   ├── src/
│   │   ├── main.rs                  # Entry point
│   │   └── lib.rs                   # Core logic, API commands, Sled DB
│   │
│   ├── Cargo.toml                   # Rust dependencies
│   ├── Cargo.lock                   # Dependency lock file
│   ├── tauri.conf.json              # Tauri configuration
│   ├── build.rs                     # Build script
│   ├── icons/                       # App icons (multiple sizes)
│   ├── capabilities/                # Tauri capabilities/permissions
│   ├── my_quiz_db/                  # Sled database storage
│   └── target/                      # Rust build artifacts
│
├── static/                           # Static assets
├── .svelte-kit/                      # SvelteKit generated files
│
├── package.json                      # Node.js dependencies & scripts
├── vite.config.js                    # Vite configuration
├── svelte.config.js                  # SvelteKit configuration
├── tsconfig.json                     # TypeScript configuration
├── .gitignore                        # Git ignore rules
└── README.md                         # This file
```

### Giải Thích Cấu Trúc

**Frontend (`src/`)**
- `routes/+page.svelte`: Component chính điều hướng giữa các màn hình
- `lib/components/`: Các component UI độc lập
- `lib/store.ts`: Global state (currentScreen, userProfile, quizResult)
- `app.css`: Tailwind directives + custom CSS variables

**Backend (`src-tauri/`)**
- `src/lib.rs`:
  - Định nghĩa `Question` struct
  - Khởi tạo Sled database
  - Expose các command: `add_question`, `get_all_questions`, `delete_question`
- `tauri.conf.json`: Window size, bundle config, app identifier
- `my_quiz_db/`: Thư mục lưu trữ database (tự động tạo)

---

## 🏗️ Kiến Trúc Hệ Thống

```
┌─────────────────────────────────────────────────────────┐
│            TAURI DESKTOP APPLICATION                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────────────────────────────────────────────────┐    │
│  │  PRESENTATION LAYER (SvelteKit SPA)            │    │
│  ├────────────────────────────────────────────────┤    │
│  │  • LoginScreen: Nhập thông tin user           │    │
│  │  • Dashboard: Trang chủ với thống kê          │    │
│  │  • QuizCategories: Chọn chủ đề                │    │
│  │  • QuizGame: Làm bài quiz                     │    │
│  │  • ResultScreen: Hiển thị kết quả             │    │
│  │                                                 │    │
│  │  Global State: Svelte Stores                   │    │
│  │    - currentScreen                             │    │
│  │    - userProfile { name, rank, unit }         │    │
│  │    - quizResult { score, total, correct }     │    │
│  └────────────────────────────────────────────────┘    │
│                        ↕ IPC                            │
│            (Tauri invoke() / Command)                   │
│                        ↕                                │
│  ┌────────────────────────────────────────────────┐    │
│  │  BUSINESS LOGIC LAYER (Rust)                   │    │
│  ├────────────────────────────────────────────────┤    │
│  │  Commands (Exposed to Frontend):               │    │
│  │    • add_question(content, a, b, correct)      │    │
│  │    • get_all_questions() → Vec<Question>       │    │
│  │    • delete_question(id)                       │    │
│  │                                                 │    │
│  │  Data Model:                                   │    │
│  │    struct Question {                           │    │
│  │      id: String,                               │    │
│  │      content: String,                          │    │
│  │      answer_a: String,                         │    │
│  │      answer_b: String,                         │    │
│  │      correct_answer: String                    │    │
│  │    }                                           │    │
│  └────────────────────────────────────────────────┘    │
│                        ↕                                │
│  ┌────────────────────────────────────────────────┐    │
│  │  DATA LAYER (Sled Embedded DB)                 │    │
│  ├────────────────────────────────────────────────┤    │
│  │  • Key-Value Store                             │    │
│  │  • Persistent Storage: my_quiz_db/             │    │
│  │  • JSON Serialization                          │    │
│  │  • No external DB server required              │    │
│  └────────────────────────────────────────────────┘    │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### Data Flow

```
User Input (Frontend)
    ↓
Component Event Handler
    ↓
Svelte Store Update (optional)
    ↓
invoke('command_name', { params })  ← IPC Call
    ↓
Rust Command Handler
    ↓
Sled DB Read/Write
    ↓
Serialize to JSON (serde)
    ↓
Return Result to Frontend
    ↓
Update UI / Store
```

---

## ⚙️ Cấu Hình

### Vite Configuration (`vite.config.js`)

```javascript
{
  server: {
    port: 1420,              // Dev server port
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"]  // Ignore Rust files
    }
  },
  plugins: [
    sveltekit(),
    tailwindcss()            // Tailwind v4 plugin
  ]
}
```

### SvelteKit Config (`svelte.config.js`)

```javascript
{
  adapter: adapter({
    fallback: 'index.html',  // SPA mode
    precompress: false
  }),
  kit: {
    // SPA configuration
  }
}
```

### Tauri Config (`src-tauri/tauri.conf.json`)

**Quan trọng:**
```json
{
  "productName": "qa-quiz-app",
  "version": "0.1.0",
  "identifier": "com.qa.quiz",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../build"
  },
  "app": {
    "windows": [{
      "title": "qa-quiz-app",
      "width": 800,
      "height": 600,
      "resizable": true,
      "fullscreen": false
    }]
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
```

### TypeScript Config (`tsconfig.json`)

```json
{
  "compilerOptions": {
    "strict": true,
    "target": "ES2020",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "esModuleInterop": true
  }
}
```

---

## 📜 Scripts & Commands

### Package.json Scripts

```bash
# Development
npm run dev              # Start Vite dev server (port 1420)
npm run tauri dev        # Start Tauri + Vite (recommended)

# Build
npm run build            # Build frontend (SvelteKit)
npm run tauri build      # Build desktop app

# Type Checking
npm run check            # Run svelte-check
npm run check:watch      # Watch mode

# Tauri CLI
npm run tauri            # Access Tauri CLI commands
npm run tauri info       # Show system info
npm run tauri icon       # Generate app icons
```

### Cargo Commands (Rust)

```bash
# Development
cargo build              # Build Rust code
cargo run                # Run Rust app
cargo test               # Run tests

# Tauri specific
cargo tauri dev          # Run Tauri dev mode
cargo tauri build        # Build production app
cargo tauri info         # System information
```

---

## 🗄️ Database Structure

### Sled Database

**Location**: `src-tauri/my_quiz_db/`

**Schema** (Stored as JSON):
```typescript
interface Question {
  id: string;              // Timestamp (milliseconds)
  content: string;         // Nội dung câu hỏi
  answer_a: string;        // Đáp án A
  answer_b: string;        // Đáp án B
  correct_answer: string;  // "A" hoặc "B"
}
```

**Storage Format**:
```
Key: "question_{timestamp}"
Value: JSON string of Question struct
```

**Example**:
```json
{
  "id": "1703001234567",
  "content": "Câu hỏi mẫu?",
  "answer_a": "Đáp án A",
  "answer_b": "Đáp án B",
  "correct_answer": "A"
}
```

---

## 🔌 API Commands

### Frontend → Backend IPC Commands

#### 1. Add Question
```typescript
import { invoke } from '@tauri-apps/api/core';

await invoke('add_question', {
  content: string,
  a: string,
  b: string,
  correct: string  // "A" or "B"
});
```

#### 2. Get All Questions
```typescript
const questions = await invoke<Question[]>('get_all_questions');
```

#### 3. Delete Question
```typescript
await invoke('delete_question', {
  id: string
});
```

### Rust Backend Implementation

**File**: `src-tauri/src/lib.rs`

```rust
#[tauri::command]
fn add_question(
    state: State<AppState>,
    content: String,
    a: String,
    b: String,
    correct: String,
) -> Result<(), String> {
    // Implementation
}

#[tauri::command]
fn get_all_questions(state: State<AppState>) -> Vec<Question> {
    // Implementation
}

#[tauri::command]
fn delete_question(state: State<AppState>, id: String) -> Result<(), String> {
    // Implementation
}
```

---

## 🎨 Theme & Styling

### Color Scheme (Military Theme)

```css
/* CSS Variables in app.css */
:root {
  --primary: #356839;           /* Army green */
  --primary-foreground: #ffffff;

  --secondary: #e6dec6;          /* Light gold */
  --secondary-foreground: #000000;

  --destructive: #ce2029;        /* Red (flag) */
  --destructive-foreground: #ffffff;

  --muted: #dbeedd;              /* Very light green */
  --muted-foreground: #356839;

  --accent: #4a7c4e;             /* Accent green */
  --accent-foreground: #ffffff;

  --background: #f4f6f4;         /* Light neutral */
  --foreground: #1a1f1a;         /* Dark text */

  --card: #ffffff;
  --card-foreground: #1a1f1a;

  --border: #c2cdc2;             /* Gray-green */
  --input: #ffffff;
  --ring: #356839;
}
```

### Tailwind Classes (Examples)

```html
<!-- Primary Button -->
<button class="bg-primary text-primary-foreground px-4 py-2 rounded-lg">
  Bắt đầu
</button>

<!-- Destructive Button -->
<button class="bg-destructive text-destructive-foreground">
  Xóa
</button>

<!-- Card -->
<div class="bg-card border border-border rounded-lg p-4">
  Nội dung
</div>
```

### Typography

**Fonts** (từ Google Fonts):
- **Work Sans**: Body text, UI elements
- **Montserrat**: Headings, emphasis

**Preload** trong `app.html`:
```html
<link rel="preconnect" href="https://fonts.googleapis.com">
<link href="https://fonts.googleapis.com/css2?family=Work+Sans:wght@400;500;600;700&display=swap" rel="stylesheet">
```

---

## 📦 Build & Deploy

### Development Build

```bash
npm run tauri dev
```

### Production Build

```bash
# Build toàn bộ
npm run tauri build

# Outputs:
# macOS: src-tauri/target/release/bundle/dmg/qa-quiz-app_0.1.0_aarch64.dmg
#        src-tauri/target/release/bundle/macos/qa-quiz-app.app
# Windows: src-tauri/target/release/bundle/msi/qa-quiz-app_0.1.0_x64_en-US.msi
# Linux: src-tauri/target/release/bundle/deb/qa-quiz-app_0.1.0_amd64.deb
#        src-tauri/target/release/bundle/appimage/qa-quiz-app_0.1.0_amd64.AppImage
```

### Build Flags (Optional)

```bash
# Release with optimizations
cargo build --release

# Specific target
cargo build --target x86_64-apple-darwin

# Clean build
cargo clean
npm run build
npm run tauri build
```

### Bundle Configuration

Customize trong `tauri.conf.json`:
```json
{
  "bundle": {
    "identifier": "com.qa.quiz",
    "category": "Education",
    "copyright": "Copyright © 2024",
    "shortDescription": "Quiz App",
    "longDescription": "Ứng dụng quiz cho quân đội"
  }
}
```

---

## 🐛 Troubleshooting

### Common Issues

#### 1. Port 1420 đã được sử dụng
```bash
# Tìm process đang dùng port
lsof -i :1420  # macOS/Linux
netstat -ano | findstr :1420  # Windows

# Kill process hoặc đổi port trong vite.config.js
```

#### 2. Rust compile errors
```bash
# Update Rust
rustup update

# Clean và rebuild
cargo clean
cargo build
```

#### 3. Node modules issues
```bash
# Xóa và cài lại
rm -rf node_modules package-lock.json
npm install
```

#### 4. Tauri CLI không tìm thấy
```bash
# Cài lại Tauri CLI
npm install -D @tauri-apps/cli@latest
```

#### 5. Database không tạo được
```bash
# Kiểm tra quyền ghi thư mục src-tauri/
# Xóa database cũ
rm -rf src-tauri/my_quiz_db
```

#### 6. Hot reload không hoạt động
```bash
# Restart dev server
# Kiểm tra vite.config.js watch settings
```

### Debug Mode

**Frontend**:
```javascript
// Mở DevTools trong app
// macOS: Cmd+Option+I
// Windows: Ctrl+Shift+I
```

**Backend**:
```rust
// Thêm debug prints
println!("Debug: {:?}", variable);

// Hoặc dùng dbg! macro
dbg!(variable);
```

### Logs Location

```bash
# Tauri logs
# macOS: ~/Library/Logs/com.qa.quiz/
# Windows: %APPDATA%\com.qa.quiz\logs\
# Linux: ~/.local/share/com.qa.quiz/logs/
```

---

## 📚 Tài Liệu Tham Khảo

### Official Documentation

- **Tauri**: https://v2.tauri.app/
- **SvelteKit**: https://kit.svelte.dev/
- **Svelte 5**: https://svelte.dev/
- **Sled Database**: https://docs.rs/sled/
- **Tailwind CSS**: https://tailwindcss.com/
- **Rust**: https://www.rust-lang.org/learn

### Useful Resources

- Tauri API Docs: https://v2.tauri.app/reference/javascript/api/
- Svelte Store: https://svelte.dev/docs/svelte-store
- TypeScript Handbook: https://www.typescriptlang.org/docs/

---

## 🤝 Contributing

### Development Workflow

1. Tạo branch mới từ `main`
```bash
git checkout -b feature/ten-tinh-nang
```

2. Develop & commit
```bash
git add .
git commit -m "feat: Thêm tính năng X"
```

3. Push và tạo Pull Request
```bash
git push origin feature/ten-tinh-nang
```

### Code Style

- **TypeScript**: Tuân theo ESLint config
- **Rust**: Chạy `cargo fmt` trước khi commit
- **Svelte**: Tuân theo Svelte style guide
- **Commits**: Sử dụng conventional commits (feat:, fix:, docs:, etc.)

---

## 📄 License

[Thêm license của bạn ở đây]

---

## 👥 Authors

[Thêm thông tin tác giả]

---

## 📞 Support

Nếu gặp vấn đề, vui lòng:
1. Kiểm tra [Troubleshooting](#-troubleshooting)
2. Tìm kiếm trong Issues
3. Tạo Issue mới với đầy đủ thông tin (OS, Node version, Rust version, error logs)

---

## 🗺️ Roadmap

- [ ] Thêm nhiều chủ đề quiz
- [ ] Export/Import câu hỏi từ file JSON
- [ ] Thêm timer cho quiz
- [ ] Lịch sử làm bài
- [ ] Multi-language support
- [ ] Dark mode
- [ ] Online sync (optional)

---

**Built with ❤️ using Tauri + SvelteKit**
