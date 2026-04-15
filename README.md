# 🌟 Prismo Tauri

[![diShine Logo](https://dishine.it/favicon.ico)](https://dishine.it)

**A professional desktop GUI for the Prismo AI Consulting Toolkit.**

_Strategize · Handle · Implement · Navigate · Evaluate_

[![Website](https://img.shields.io/badge/Website-dishine.it-blue)](https://dishine.it/)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-diShine-blue)](https://linkedin.com/company/100682596)
[![Location](https://img.shields.io/badge/Location-Milan%2C%20Italy-green)](https://dishine.it)
[![License: CC0](https://img.shields.io/badge/License-CC0%201.0-lightgrey)](./LICENSE)

<p align="center">
  <img src="assets/branding/prismoTauri_01_dashboard_close.webp" alt="Prismo Tauri Dashboard dashboard" width="65%">
</p>

Prismo Tauri transforms the [Prismo](https://github.com/diShine-digital-agency/prismo) CLI toolkit by diShine into a cross-platform desktop application built with [Tauri v2](https://tauri.app), React, and TypeScript. It provides a visual interface for running audits, viewing reports, exporting branded documents, and managing client profiles.

<p align="center">
  <img src="assets/branding/prismoTauri_04_report_viewer_close.webp" alt="report viewer" width="49%">
  <img src="assets/branding/prismoTauri_05_export_center_close.webp" alt="Export center" width="49%">
</p>

Built by [diShine Digital Agency](https://dishine.it).

> 📖 **New here?** Start with the [Complete User Guide](docs/GUIDE.md) — it walks you through everything step by step, no technical knowledge required.

<p align="center">
  <img src="assets/branding/prismoTauri_02_run_audit_close.webp" alt="Run audit" width="65%">
</p>

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| **[Complete User Guide](docs/GUIDE.md)** | Step-by-step guide for all users (beginners to experts) |
| **[Changelog](CHANGELOG.md)** | Version history and release notes |
| **[Contributing](CONTRIBUTING.md)** | Developer setup, code style, and contribution guidelines |
| **[Security Policy](SECURITY.md)** | Vulnerability reporting and security details |
| **[Brand Guidelines](assets/branding/BRAND.md)** | Colors, typography, and brand assets |
| **[Export Guide](toolkit/export/README.md)** | Report export formats and CLI tools |
| **[Sample Report](docs/examples/sample-report-website-performance.md)** | Example of a generated audit report |
| **[Sample Client Profile](docs/examples/sample-client-profile.json)** | Example client profile data |

---

## Features

### 🖥️ Desktop Application
- Native desktop app for Windows, macOS, and Linux
- Professional dark-theme UI with Prismo branding
- Lightweight binary (~3-10 MB + WebView2)
- USB-portable architecture

### 🔍 19 Audit Categories
| Category | Audits | New in Tauri |
|----------|--------|--------------|
| System Health | Windows, Linux, macOS diagnosis, Log analysis, Network diagnostics | — |
| Web & Performance | Website performance, Tech stack analysis, Accessibility | — |
| SEO | Technical SEO, On-page SEO, Competitive analysis | — |
| MarTech & Data | MarTech stack, Data quality | — |
| Security | Website security, System security | — |
| Email & DNS | SPF, DKIM, DMARC, MX records, DNS security | ✅ |
| Privacy | GDPR compliance, Cookie consent, Privacy policy | ✅ |
| Social | Open Graph, Twitter Cards, Schema.org structured data | ✅ |
| API | API security, CORS, Rate limiting, Auth | ✅ |

### 📤 Branded Report Export
Export audit reports in multiple formats — all with professional branding:

| Format | Branded | Printable | Method |
|--------|---------|-----------|--------|
| **PDF** | ✅ Cover page, logo, colors, footer | ✅ | Print dialog or md-to-pdf |
| **HTML** | ✅ Full CSS styling, XSS-safe | ✅ | One-click export |
| **Markdown** | ✅ Header/footer branding | via viewer | One-click export |
| **Plain Text** | ASCII text | ✅ | One-click export |

### 📄 Reports
- View and search saved audit reports
- Loads reports from backend filesystem with fallback to sample data
- One-click to open any report in the full Report Viewer

### 👥 Client Manager
- Create and manage client profiles
- Store context (domain, industry, tech stack, competitors)
- Search and filter clients by name, domain, or industry
- Client data persists across sessions
- Richer audit results with client-specific context

### ⚙️ Settings
- Language selection (English, Italian, French)
- API key management (stored securely in client-side storage)
- White-label mode (rebrand with your agency name)
- Theme configuration (dark mode)
- Settings persist across sessions

---

## Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Framework | Tauri v2 | Native desktop app with Rust backend |
| Frontend | React 19 + TypeScript | UI components |
| Styling | Tailwind CSS v4 | Utility-first CSS |
| Markdown | react-markdown | Report rendering |
| Build | Vite 7 | Fast bundling |
| Backend | Rust (Tauri) | File ops, system commands, process management |

---

## Getting Started

### Prerequisites
- [Node.js 22+](https://nodejs.org)
- [Rust](https://rustup.rs)
- Platform-specific dependencies:
  - **Linux**: `libwebkit2gtk-4.1-dev librsvg2-dev libgtk-3-dev libsoup-3.0-dev`
  - **macOS**: Xcode Command Line Tools
  - **Windows**: WebView2 (pre-installed on Windows 10/11)

### Development

```bash
# Navigate to the project
cd prismo-tauri

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

### Build for Production

```bash
# Build optimized binary
npm run tauri build
```

The output binary will be in `src-tauri/target/release/`.

---

## Project Structure

```
prismo-tauri/
├── src/                        # Frontend (React + TypeScript)
│   ├── components/
│   │   ├── Sidebar.tsx         # Navigation sidebar
│   │   ├── AuditCard.tsx       # Audit option cards
│   │   └── ErrorBoundary.tsx   # Error catching boundary
│   ├── constants.ts            # Centralized version constant
│   ├── pages/
│   │   ├── Dashboard.tsx       # Overview, quick-start
│   │   ├── AuditRunner.tsx     # Select & run audits
│   │   ├── Reports.tsx         # View past reports
│   │   ├── ReportViewer.tsx    # Render & export reports
│   │   ├── ClientManager.tsx   # Client profile management
│   │   ├── Settings.tsx        # App configuration
│   │   └── ExportCenter.tsx    # Export format info & tools
│   ├── App.tsx                 # Root component with routing
│   ├── main.tsx                # React entry point
│   └── index.css               # Tailwind CSS + brand styles
├── src-tauri/                  # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs              # Tauri commands (audit, config, reports)
│   │   └── main.rs             # Entry point
│   ├── Cargo.toml              # Rust dependencies
│   ├── tauri.conf.json         # Tauri app configuration
│   └── capabilities/           # Security permissions
├── toolkit/
│   ├── prompts/                # All 19 audit prompt templates
│   │   ├── system/             # 5 system health prompts
│   │   ├── web/                # 3 web performance prompts
│   │   ├── seo/                # 3 SEO prompts
│   │   ├── martech/            # 2 MarTech prompts
│   │   ├── security/           # 2 security prompts
│   │   ├── email-dns/          # 1 email/DNS prompt
│   │   ├── privacy/            # 1 GDPR/privacy prompt
│   │   ├── social/             # 1 social media prompt
│   │   └── api/                # 1 API security prompt
│   └── export/                 # Report export templates
│       ├── report-style.css    # Branded CSS for PDF export
│       └── README.md           # Export documentation
├── assets/
│   └── branding/               # Brand assets & guidelines
│       ├── BRAND.md            # Brand guidelines
│       └── colors.json         # Color palette definition
├── docs/
│   ├── GUIDE.md                # Complete step-by-step user guide
│   └── examples/               # Sample outputs
│       ├── sample-client-profile.json
│       └── sample-report-website-performance.md
├── public/
│   └── prismo-icon.svg         # App icon
├── package.json
├── vite.config.ts
├── tsconfig.json
├── CHANGELOG.md                # Version history
├── CONTRIBUTING.md             # Developer guide
├── SECURITY.md                 # Security policy
└── LICENSE                     # MIT License
```

---

## Brand Colors

| Name | Hex | Usage |
|------|-----|-------|
| Prismo Purple | `#6C5CE7` | Primary brand color |
| Teal Accent | `#00CEC9` | Secondary highlights |
| Charcoal | `#2D3436` | Body text, dark backgrounds |
| Success Green | `#00B894` | Passed checks |
| Warning Amber | `#FDCB6E` | Medium severity |
| Danger Coral | `#E17055` | High severity |
| Critical Red | `#D63031` | Critical severity |

---

## Relationship to Prismo CLI

Prismo Tauri is a **separate project** that wraps the original [Prismo CLI toolkit](https://github.com/diShine-digital-agency/prismo) in a desktop GUI. It:

- Includes all 15 original audit prompts + 4 new categories
- Adds branded report export (PDF, HTML, Markdown, TXT)
- Provides a visual client profile manager
- Offers a settings panel with white-label support
- Does **not** modify the original Prismo repository

The CLI version remains the primary tool for USB-based on-site audits. Prismo Tauri is designed for consultants who prefer a GUI workflow or need branded report delivery.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development guidelines.

## Security

See [SECURITY.md](SECURITY.md) for our security policy and vulnerability reporting.

## License

MIT License — see [LICENSE](LICENSE) for details.

Copyright (c) 2026 [diShine Digital Agency](https://dishine.it)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.
