# Cross-Platform Architecture Scaffold: Tauri v2 + Vue 3 + Rust

> **Companion Demo Repo**  
> This repository is a minimal architectural scaffold demonstrating how to structure a cross-platform Rust and Vue 3 monorepo. It is designed to accompany the blog article:  
> 🔗 **[Building a Cross-Platform App with Tauri v2 and Vue 3](https://example.com/blog/cross-platform-rust-tauri-v2-vue3)** *(placeholder)*

---

## What This Demonstrates

This repository is **not** an end-user application; it is a minimal architectural blueprint illustrating:

1. **A Unified Monorepo**: Combining a Vue 3 + Vite frontend with a multi-crate Rust workspace (`crates/common`, `crates/api`, and `crates/tauri`).
2. **Compile-Time Platform Switching**: Using Vite module aliases (`@platform`) rather than runtime conditionals (`if (window.__TAURI__)`) or ambient global constants.
3. **Platform Service Decoupling**: Sharing identical TypeScript interfaces (`IStorageService`, `ISessionService`) across Web/PWA and native Tauri runtimes.
4. **Shared Rust Domain Core**: Sharing data structures (e.g. `AppInfo`) between a native desktop/mobile shell (`tauri`) and a standalone backend server (`api` via Axum).
5. **Mobile-Ready Dev Workflow**: Dev server configuration accommodating `TAURI_DEV_HOST` and WebSocket HMR for mobile device testing.

---

## The `@platform` Alias in Action

Rather than scattering conditional checks (`isTauri ? ... : ...`) or relying on globals throughout UI components, the architecture decouples platform-specific implementations behind a unified `@platform` alias. In `vite.config.ts`, `@platform` is resolved to `src/platform/web` (providing web and PWA implementations alongside `vite-plugin-pwa`), while in `vite.config.tauri.ts`, `@platform` maps to `src/platform/tauri` (providing native Tauri `invoke` bindings). Because both modules strictly implement the contracts defined in `src/platform/types.ts` and export the same shape (`isTauri`, `storage`, `session`), UI components import from `@platform` with full type safety and zero runtime overhead, enabling tree-shaking and platform-specific bundle optimization.

---

## Monorepo Layout

```text
.
├── apps/
│   └── app/                      # Vue 3 + TypeScript + Vite frontend
│       ├── src/
│       │   ├── platform/
│       │   │   ├── types.ts      # Shared IStorageService, ISessionService contracts
│       │   │   ├── web/          # Web/PWA implementation (isTauri = false, fetch stubs)
│       │   │   └── tauri/        # Tauri implementation (isTauri = true, invoke stubs)
│       │   └── views/
│       │       └── HomeView.vue  # Component consuming @platform
│       ├── vite.config.ts        # Web & PWA config (@platform -> src/platform/web)
│       └── vite.config.tauri.ts  # Tauri shell config (@platform -> src/platform/tauri)
├── crates/
│   ├── api/                      # Minimal Axum HTTP server (health route)
│   ├── common/                   # Shared types (AppInfo) used by api & tauri
│   ├── tauri/                    # Tauri v2 native desktop & mobile shell
│   │   ├── capabilities/         # Tauri v2 capability definitions
│   │   └── tauri.conf.json       # App configuration and build scripts
│   └── .taurignore               # Ignores api/ from triggering Tauri dev rebuilds
├── Cargo.toml                    # Rust workspace
└── pnpm-workspace.yaml           # Frontend workspace
```

---

## Getting Started

### Prerequisites

- **Node.js**: v18+ (v20+ recommended)
- **pnpm**: v9+ (`corepack enable pnpm`)
- **Rust**: Stable toolchain (`rustup update stable`)
- **System dependencies for Tauri**: See [Tauri Prerequisites](https://v2.tauri.app/start/prerequisites/) for your operating system.

Install dependencies across the monorepo:

```bash
pnpm install
```

---

## Running Each Target

### 1. Web Target (PWA)

Runs Vite with the web configuration (`vite.config.ts`), including `vite-plugin-pwa`:

```bash
# From workspace root:
pnpm dev

# Or directly in apps/app:
cd apps/app && pnpm dev
```

Build for web production:

```bash
pnpm build
```

### 2. Tauri Desktop Target

Launches the Tauri v2 desktop application in development mode with hot-reloading:

```bash
# From workspace root:
pnpm tauri dev

# Or directly in apps/app:
cd apps/app && pnpm tauri dev
```

Build the native desktop application binary:

```bash
pnpm tauri build
```

### 3. Tauri Mobile Target

To run on iOS or Android, ensure platform prerequisites are installed:

- **Android**: Android Studio, Android SDK Platform-Tools, NDK, and `ANDROID_HOME` configured. Run `rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android`.
- **iOS** (macOS only): Xcode and iOS Simulator. Run `rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim`.

Initialize mobile platforms if needed:
```bash
cd apps/app
pnpm tauri android init
# or
pnpm tauri ios init
```

Run on mobile emulator / device:
```bash
cd apps/app
pnpm tauri android dev
# or
pnpm tauri ios dev
```

> **Note on Mobile HMR**: `vite.config.tauri.ts` inspects `process.env.TAURI_DEV_HOST`. During mobile dev, Tauri automatically binds the local IP to allow the simulator or device to connect to the dev server's WebSocket on port `1421`.

### 4. Axum Backend API

The minimal Axum server demonstrates sharing `crates/common` types with a backend server:

```bash
cargo run -p api
```

Verify the health check endpoint:
```bash
curl http://localhost:3000/health
# {"app":{"name":"horatio-scaffold","version":"0.1.0"},"status":"ok"}
```

---

## Notes on Architecture & Boundaries

- **No backend DB or Auth**: Kept intentionally minimal as a teaching repository.
- **Tauri ignores API crate**: `.taurignore` explicitly excludes `crates/api` from triggering rebuilds when developing the Tauri application.
- **Tauri build output**: The `crates/tauri/gen/` directory generated by Tauri build scripts is ignored in `.gitignore`.
